use crate::{x01_beam_pessimistic, AiModel, FastRng, Game, State};

#[derive(Clone, Copy, Debug)]
struct RacingStat {
    sum: f64,
    sum2: f64,
    count: usize,
    downside_count: usize,
}

impl RacingStat {
    fn new() -> Self {
        Self {
            sum: 0.0,
            sum2: 0.0,
            count: 0,
            downside_count: 0,
        }
    }

    fn push(&mut self, v: f64, is_downside: bool) {
        self.sum += v;
        self.sum2 += v * v;
        self.count += 1;
        if is_downside {
            self.downside_count += 1;
        }
    }

    fn mean(self) -> f64 {
        if self.count == 0 {
            return f64::NEG_INFINITY;
        }
        self.sum / self.count as f64
    }

    fn std(self) -> f64 {
        if self.count <= 1 {
            return 0.0;
        }
        let mean = self.mean();
        let var = (self.sum2 / self.count as f64 - mean * mean).max(0.0);
        var.sqrt()
    }

    fn stderr(self) -> f64 {
        if self.count <= 1 {
            return f64::INFINITY;
        }
        self.std() / (self.count as f64).sqrt()
    }

    fn downside_prob(self) -> f64 {
        if self.count == 0 {
            return 1.0;
        }
        self.downside_count as f64 / self.count as f64
    }
}

pub(super) fn choose_move_x05_adaptive_racing(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    // x05は中人数帯を対象にし、それ以外はx01の安定版を再利用する。
    if !(3..=5).contains(&game.m) {
        return x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
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
    let max_rounds = if game.m == 5 { 12 } else { 14 };
    let min_rounds = 3usize;
    let confidence_z = 1.05_f64;
    let base_score = crate::strategic_score(game, state);
    let downside_drop = if game.m == 5 { 4200.0 } else { 3200.0 };

    let ai_options = crate::build_ai_candidates_and_probs(game, state, models);
    let seed = ((state.turn as u64 + 1) * 0x9e37_79b9_7f4a_7c15)
        ^ (scores[0] as u64)
        ^ ((game.m as u64) << 32)
        ^ ((game.u as u64) << 48)
        ^ 0xa5a5_5a5a_cc33_33cc;
    let mut rng = FastRng::new(seed);

    let mut active: Vec<usize> = (0..candidate_cap).collect();
    let mut stats = vec![RacingStat::new(); candidate_cap];
    for round in 0..max_rounds {
        let mut sampled_ai_moves = Vec::with_capacity(game.m.saturating_sub(1));
        for (cands, probs) in &ai_options {
            let idx = crate::sample_index(probs, &mut rng);
            sampled_ai_moves.push(cands[idx]);
        }

        for &idx in &active {
            let mv = ranked[idx].0;
            let mut sampled = Vec::with_capacity(game.m);
            sampled.push(mv);
            sampled.extend_from_slice(&sampled_ai_moves);
            let next_state = crate::simulate_turn(game, state, &sampled);
            let v = crate::strategic_score(game, &next_state);
            let is_downside = v + downside_drop < base_score;
            stats[idx].push(v, is_downside);
        }

        if round + 1 < min_rounds || active.len() <= 3 {
            continue;
        }

        let mut best_lcb = f64::NEG_INFINITY;
        for &idx in &active {
            let st = stats[idx];
            let mean = st.mean();
            let se = st.stderr();
            let lcb = if se.is_finite() {
                mean - confidence_z * se
            } else {
                mean - 1.5 * st.std()
            };
            if lcb > best_lcb {
                best_lcb = lcb;
            }
        }
        let prune_margin = 900.0 + 0.0020 * best_lcb.abs();
        let mut kept = Vec::with_capacity(active.len());
        for &idx in &active {
            let st = stats[idx];
            let mean = st.mean();
            let se = st.stderr();
            let ucb = if se.is_finite() {
                mean + confidence_z * se
            } else {
                mean + 1.5 * st.std()
            };
            if ucb + prune_margin >= best_lcb {
                kept.push(idx);
            }
        }
        if kept.len() < 2 {
            let mut by_mean = active.clone();
            by_mean.sort_by(|&a, &b| {
                stats[b]
                    .mean()
                    .partial_cmp(&stats[a].mean())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            kept = by_mean.into_iter().take(2).collect();
        }
        active = kept;
        if active.len() == 1 {
            break;
        }
    }

    let risk_w = if game.m == 5 { 0.24 } else { 0.22 };
    let downside_w = 480.0;
    let local_w = 0.09;
    let mut best_idx = active[0];
    let mut best_total = f64::NEG_INFINITY;
    for &idx in &active {
        let st = stats[idx];
        let mean = st.mean();
        let std = st.std();
        let downside_prob = st.downside_prob();
        let local = ranked[idx].1;
        let total = mean - risk_w * std - downside_w * downside_prob + local_w * local;
        if total > best_total {
            best_total = total;
            best_idx = idx;
        }
    }
    ranked[best_idx].0
}
