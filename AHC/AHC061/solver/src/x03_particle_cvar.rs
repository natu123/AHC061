use crate::{x06_expert_switch_hybrid, AiModel, FastRng, Game, State};

#[derive(Clone)]
struct ParticleBank {
    candidates: Vec<(usize, usize)>,
    probs_by_particle: Vec<Vec<f64>>,
    particle_weights: Vec<f64>,
}

fn normalize_probs(mut probs: Vec<f64>) -> Vec<f64> {
    if probs.is_empty() {
        return probs;
    }
    let sum: f64 = probs.iter().copied().sum();
    if !sum.is_finite() || sum <= 0.0 {
        return vec![1.0 / probs.len() as f64; probs.len()];
    }
    for p in &mut probs {
        *p = (*p / sum).clamp(0.0, 1.0);
    }
    probs
}

fn make_particle_model(base: &AiModel, weight_scale: [f64; 4], eps_shift: f64) -> AiModel {
    let mut model = base.clone();
    for (k, scale) in weight_scale.into_iter().enumerate() {
        model.w[k] = (model.w[k] * scale).clamp(0.08, 2.30);
    }
    model.eps_est = (model.eps_est + eps_shift).clamp(0.05, 0.70);
    model
}

fn particle_models_and_weights(base: &AiModel) -> (Vec<AiModel>, Vec<f64>) {
    let confidence = base.seen as f64 / (base.seen as f64 + 12.0);
    let uncertainty = base.eps_est.clamp(0.05, 0.70);

    let models = vec![
        make_particle_model(base, [1.00, 1.00, 1.00, 1.00], 0.00),
        make_particle_model(base, [1.12, 0.94, 1.20, 0.86], 0.03),
        make_particle_model(base, [0.94, 1.14, 0.90, 1.18], 0.09),
        make_particle_model(base, [1.18, 0.90, 1.08, 0.78], -0.04),
        make_particle_model(base, [0.86, 0.86, 0.86, 0.86], 0.18),
    ];

    let base_w = 0.30 + 0.22 * confidence * (1.0 - uncertainty);
    let attack_w = 0.17 + 0.08 * (1.0 - uncertainty);
    let defense_w = 0.17 + 0.07 * uncertainty;
    let greedy_w = 0.14 + 0.06 * confidence;
    let noisy_w = 0.10 + 0.28 * (1.0 - confidence) + 0.20 * uncertainty;
    let weights = normalize_probs(vec![base_w, attack_w, defense_w, greedy_w, noisy_w]);

    (models, weights)
}

fn build_particle_bank(game: &Game, state: &State, player: usize, base: &AiModel) -> ParticleBank {
    let candidates = crate::get_candidates(game, state, player);
    if candidates.is_empty() {
        return ParticleBank {
            candidates: vec![state.pos[player]],
            probs_by_particle: vec![vec![1.0]],
            particle_weights: vec![1.0],
        };
    }

    let (particles, weights) = particle_models_and_weights(base);
    let mut probs_by_particle = Vec::with_capacity(particles.len());
    for model in &particles {
        let probs = crate::blended_ai_probs(game, state, player, model, &candidates);
        probs_by_particle.push(normalize_probs(probs));
    }

    ParticleBank {
        candidates,
        probs_by_particle,
        particle_weights: weights,
    }
}

fn cvar_lower(samples: &mut [f64], alpha: f64) -> f64 {
    if samples.is_empty() {
        return f64::NEG_INFINITY;
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let tail_count = ((samples.len() as f64 * alpha).ceil() as usize)
        .max(1)
        .min(samples.len());
    let mut sum = 0.0;
    for &v in samples.iter().take(tail_count) {
        sum += v;
    }
    sum / tail_count as f64
}

pub(super) fn choose_move_x03_particle_cvar(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    // 初版観測では中人数帯で優位だったため、適用帯を限定する。
    if !(3..=5).contains(&game.m) {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let candidates = crate::get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let cur = state.pos[0];
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let mut ranked: Vec<((usize, usize), f64)> = Vec::with_capacity(candidates.len());
    for &mv in &candidates {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict_map,
            cur,
            &is_leader,
        );
        ranked.push((mv, local));
    }
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let candidate_cap = if ranked.len() >= 24 {
        14
    } else if ranked.len() >= 14 {
        10
    } else {
        ranked.len()
    };
    let sample_count = if game.m >= 5 { 16 } else { 12 };
    let tail_alpha = if game.m >= 5 { 0.30 } else { 0.25 };
    let cvar_w = if game.m >= 5 { 0.58 } else { 0.45 };
    let local_w = 0.07;

    let mut particle_banks = Vec::with_capacity(game.m.saturating_sub(1));
    for ai_idx in 0..game.m.saturating_sub(1) {
        particle_banks.push(build_particle_bank(game, state, ai_idx + 1, &models[ai_idx]));
    }

    let seed = ((state.turn as u64 + 1) * 0x517c_c1b7_2722_0a95)
        ^ (scores[0] as u64)
        ^ ((game.m as u64) << 32)
        ^ ((game.u as u64) << 48)
        ^ 0x3f15_9d7e_a42c_55b1;
    let mut rng = FastRng::new(seed);

    let mut best_mv = ranked[0].0;
    let mut best_val = f64::NEG_INFINITY;
    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let mut samples = Vec::with_capacity(sample_count);
        for _ in 0..sample_count {
            let mut sampled_moves = Vec::with_capacity(game.m);
            sampled_moves.push(mv);
            for bank in &particle_banks {
                let pidx = crate::sample_index(&bank.particle_weights, &mut rng);
                let probs = &bank.probs_by_particle[pidx];
                let midx = crate::sample_index(probs, &mut rng);
                sampled_moves.push(bank.candidates[midx]);
            }
            let next_state = crate::simulate_turn(game, state, &sampled_moves);
            samples.push(crate::strategic_score(game, &next_state));
        }

        let mean = samples.iter().copied().sum::<f64>() / sample_count as f64;
        let cvar = cvar_lower(&mut samples, tail_alpha);
        let total = (1.0 - cvar_w) * mean + cvar_w * cvar + local_w * local;
        if total > best_val {
            best_val = total;
            best_mv = mv;
        }
    }
    best_mv
}
