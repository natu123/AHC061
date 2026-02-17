use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x07_dual_horizon_route, x08_pressure_frontier, x09_regret_mix, AiModel, Game, State,
};

fn advisor_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
) -> Vec<((usize, usize), f64)> {
    let mut votes = Vec::<((usize, usize), f64)>::new();
    let mut add_vote = |mv: (usize, usize), w: f64| {
        votes.push((mv, w));
    };

    if game.m == 4 {
        add_vote(x04_macro_route::choose_move_x04_macro_route(game, state, models), 1.35 + 0.15 * (1.0 - phase));
    }
    add_vote(
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
        1.00 + 0.10 * uncertainty,
    );
    add_vote(
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
        0.85 + 0.10 * (1.0 - phase),
    );
    if (3..=5).contains(&game.m) {
        add_vote(
            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
            0.80 + 0.20 * uncertainty,
        );
    }
    add_vote(
        x07_dual_horizon_route::choose_move_x07_dual_horizon_route(game, state, models),
        0.90,
    );
    add_vote(
        x08_pressure_frontier::choose_move_x08_pressure_frontier(game, state, models),
        0.80,
    );
    add_vote(x09_regret_mix::choose_move_x09_regret_mix(game, state, models), 0.95);
    if phase <= 0.70 {
        add_vote(
            x09_regret_mix::choose_move_x09_regret_mix(game, state, models),
            0.35,
        );
    }

    votes
}

fn collect_candidate_moves(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
) -> Vec<(usize, usize)> {
    let mut ranked_advisors = advisor_votes(game, state, models, phase, uncertainty);
    ranked_advisors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut candidates = Vec::<(usize, usize)>::new();
    for (mv, _) in ranked_advisors {
        if !candidates.contains(&mv) {
            candidates.push(mv);
        }
        if candidates.len() >= 8 {
            break;
        }
    }

    let scores = crate::calc_scores(game, state);
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let s0 = scores[0] as f64;
    let conflict = crate::estimate_conflict_map(game, state, models);
    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }

    for &(mv, local) in &crate::get_candidates(game, state, 0)
        .iter()
        .map(|&mv| {
            (
                mv,
                crate::evaluate_local_move(
                    game,
                    state,
                    mv,
                    &scores,
                    s0,
                    max_ai_i64,
                    phase,
                    &conflict,
                    state.pos[0],
                    &leaders,
                ),
            )
        })
        .collect::<Vec<_>>()
    {
        let _ = local;
        if candidates.len() < 14 && !candidates.contains(&mv) {
            candidates.push(mv);
        }
    }
    if candidates.is_empty() {
        candidates.push(state.pos[0]);
    }
    candidates
}

pub(super) fn choose_move_x12_advisor_vote_ensemble(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let phase = state.turn as f64 / game.t as f64;
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let leaders = {
        let mut flags = vec![false; game.m];
        for p in 1..game.m {
            if scores[p] == max_ai_i64 {
                flags[p] = true;
            }
        }
        flags
    };

    let advisor_votes = advisor_votes(game, state, models, phase, uncertainty);
    let candidates = collect_candidate_moves(game, state, models, phase, uncertainty);
    let mut best_mv = candidates[0];
    let mut best_val = f64::NEG_INFINITY;

    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
        3
    } else if game.m >= 6 && uncertainty >= 0.28 {
        2
    } else {
        1
    };
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, secondary_cap);
    let base_gap = (max_ai_i64 as f64 - scores[0] as f64).max(0.0) / s0.max(1.0);

    for &mv in &candidates {
        let mut vote_weight = 0.0_f64;
        for &(av, w) in &advisor_votes {
            if av == mv {
                vote_weight += w;
            }
        }
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict,
            state.pos[0],
            &leaders,
        );
        let mut moves1 = Vec::with_capacity(game.m);
        moves1.push(mv);
        moves1.extend_from_slice(&primary);
        let mut moves2 = Vec::with_capacity(game.m);
        moves2.push(mv);
        moves2.extend_from_slice(&secondary);

        let ns1 = crate::simulate_turn(game, state, &moves1);
        let ns2 = crate::simulate_turn(game, state, &moves2);
        let s1 = crate::strategic_score(game, &ns1);
        let s2 = crate::strategic_score(game, &ns2);
        let frontier_penalty = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
        let recover_boost = (1.0 - phase) * 12.0 + (1.0 - uncertainty) * 8.0;
        let vote_pressure = (0.70 + 0.10 * uncertainty) * vote_weight;

        let total = (0.68 + 0.12 * (1.0 - base_gap)) * s1
            + (0.32 - 0.12 * (1.0 - base_gap)) * s2
            + 0.12 * local
            + 0.50 * vote_pressure
            + recover_boost
            - frontier_penalty
            + if scores[0] < max_ai_i64 { 18.0 } else { 0.0 };
        if total > best_val {
            best_val = total;
            best_mv = mv;
        }
    }
    best_mv
}
