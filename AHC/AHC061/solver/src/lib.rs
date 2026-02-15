use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

mod strategy_mode;
mod x01_beam_pessimistic;
mod x02_monte_carlo;
mod x05_adaptive_racing_mc;
mod x06_expert_switch_hybrid;

pub use strategy_mode::{strategy_from_env, StrategyMode};
use strategy_mode::choose_move;

#[derive(Clone)]
pub(crate) struct Game {
    n: usize,
    m: usize,
    t: usize,
    u: usize,
    v: Vec<Vec<i64>>,
}

#[derive(Clone)]
pub(crate) struct State {
    pos: Vec<(usize, usize)>,
    owner: Vec<Vec<i32>>,
    level: Vec<Vec<usize>>,
    turn: usize,
}

#[derive(Clone)]
pub(crate) struct AiModel {
    w: [f64; 4],
    eps_est: f64,
    seen: u32,
    mismatch: u32,
}

impl AiModel {
    fn new() -> Self {
        Self {
            w: [0.64, 0.64, 0.64, 0.64],
            eps_est: 0.30,
            seen: 0,
            mismatch: 0,
        }
    }
}

pub(crate) struct FastRng {
    state: u64,
}

impl FastRng {
    pub(crate) fn new(seed: u64) -> Self {
        Self { state: seed | 1 }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
        self.state ^= self.state << 8;
        self.state
    }

    pub(crate) fn next_f64(&mut self) -> f64 {
        let x = self.next_u64() >> 11;
        (x as f64) * (1.0 / ((1_u64 << 53) as f64))
    }
}

struct Scanner<R: BufRead> {
    reader: R,
    line: String,
    tokens: VecDeque<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            line: String::new(),
            tokens: VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(tok) = self.tokens.pop_front() {
                if let Ok(v) = tok.parse::<T>() {
                    return Some(v);
                }
                return None;
            }
            self.line.clear();
            let n = self.reader.read_line(&mut self.line).ok()?;
            if n == 0 {
                return None;
            }
            let s = self.line.trim();
            if s.is_empty() || s.starts_with('#') {
                continue;
            }
            self.tokens = s
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect::<VecDeque<_>>();
        }
    }
}

pub(crate) fn in_bounds(n: usize, x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < n && (y as usize) < n
}

pub(crate) fn occupied_by_other(state: &State, player: usize, x: usize, y: usize) -> bool {
    for (i, &(px, py)) in state.pos.iter().enumerate() {
        if i != player && px == x && py == y {
            return true;
        }
    }
    false
}

pub(crate) fn get_candidates(game: &Game, state: &State, player: usize) -> Vec<(usize, usize)> {
    let mut reachable = Vec::new();
    let mut visited = vec![vec![false; game.n]; game.n];
    let mut q = VecDeque::new();

    let start = state.pos[player];
    q.push_back(start);
    visited[start.0][start.1] = true;

    // tools/src/lib.rs の近傍順を維持。
    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y)) = q.pop_front() {
        if !occupied_by_other(state, player, x, y) {
            reachable.push((x, y));
        }

        if state.owner[x][y] == player as i32 {
            for (dx, dy) in DIRS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if in_bounds(game.n, nx, ny) {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if !visited[ux][uy] {
                        visited[ux][uy] = true;
                        q.push_back((ux, uy));
                    }
                }
            }
        }
    }
    reachable
}

pub(crate) fn calc_scores(game: &Game, state: &State) -> Vec<i64> {
    let mut scores = vec![0_i64; game.m];
    for i in 0..game.n {
        for j in 0..game.n {
            let owner = state.owner[i][j];
            if owner >= 0 {
                scores[owner as usize] += game.v[i][j] * state.level[i][j] as i64;
            }
        }
    }
    scores
}

pub(crate) fn ai_features(
    game: &Game,
    state: &State,
    player: usize,
    target: (usize, usize),
) -> [f64; 4] {
    let (x, y) = target;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;

    if owner == -1 {
        [value, 0.0, 0.0, 0.0]
    } else if owner == player as i32 {
        if level < game.u {
            [0.0, value, 0.0, 0.0]
        } else {
            [0.0, 0.0, 0.0, 0.0]
        }
    } else if level == 1 {
        [0.0, 0.0, value, 0.0]
    } else {
        [0.0, 0.0, 0.0, value]
    }
}

pub(crate) fn dot4(w: &[f64; 4], x: &[f64; 4]) -> f64 {
    w[0] * x[0] + w[1] * x[1] + w[2] * x[2] + w[3] * x[3]
}

pub(crate) fn predict_ai_distribution(
    game: &Game,
    state: &State,
    player: usize,
    model: &AiModel,
    candidates: &[(usize, usize)],
) -> Vec<f64> {
    if candidates.is_empty() {
        return Vec::new();
    }

    let mut est_scores = vec![0.0_f64; candidates.len()];
    for (i, &cand) in candidates.iter().enumerate() {
        let feat = ai_features(game, state, player, cand);
        est_scores[i] = dot4(&model.w, &feat);
    }

    let max_score = est_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let tol = 1e-9 * max_score.abs().max(1.0);
    let best_idx: Vec<usize> = (0..candidates.len())
        .filter(|&i| est_scores[i] >= max_score - tol)
        .collect();

    let eps = model.eps_est.clamp(0.05, 0.60);
    let base = eps / candidates.len() as f64;
    let mut probs = vec![base; candidates.len()];
    let rem = 1.0 - eps;
    let share = if best_idx.is_empty() {
        rem / candidates.len() as f64
    } else {
        rem / best_idx.len() as f64
    };

    if best_idx.is_empty() {
        for p in &mut probs {
            *p += share;
        }
    } else {
        for &i in &best_idx {
            probs[i] += share;
        }
    }
    probs
}

pub(crate) fn blended_ai_probs(
    game: &Game,
    state: &State,
    player: usize,
    model: &AiModel,
    candidates: &[(usize, usize)],
) -> Vec<f64> {
    if candidates.is_empty() {
        return Vec::new();
    }
    let model_probs = predict_ai_distribution(game, state, player, model, candidates);
    let uniform_prob = 1.0 / candidates.len() as f64;
    let turns_ratio = (state.turn as f64 / game.t as f64).clamp(0.0, 1.0);
    let seen = model.seen as f64;
    let confidence = (seen / (seen + 10.0)) * (1.0 - model.eps_est).clamp(0.15, 0.95);
    let alpha = (0.05 + 0.90 * turns_ratio * confidence).clamp(0.05, 0.95);
    model_probs
        .iter()
        .map(|&p| alpha * p + (1.0 - alpha) * uniform_prob)
        .collect()
}

pub(crate) fn estimate_conflict_map(game: &Game, state: &State, models: &[AiModel]) -> Vec<Vec<f64>> {
    let mut map = vec![vec![0.0_f64; game.n]; game.n];
    for ai_idx in 0..(game.m.saturating_sub(1)) {
        let player = ai_idx + 1;
        let cands = get_candidates(game, state, player);
        if cands.is_empty() {
            continue;
        }
        let probs = blended_ai_probs(game, state, player, &models[ai_idx], &cands);
        for (i, &(x, y)) in cands.iter().enumerate() {
            map[x][y] += probs[i];
        }
    }
    map
}

pub(crate) fn simulate_turn(game: &Game, state: &State, moves: &[(usize, usize)]) -> State {
    let mut next = state.clone();
    let mut temp_pos = moves.to_vec();
    let mut move_counts = HashMap::<(usize, usize), usize>::new();
    for &mv in moves {
        *move_counts.entry(mv).or_insert(0) += 1;
    }

    let mut collected = vec![false; game.m];
    for i in 0..game.m {
        let target = temp_pos[i];
        if move_counts[&target] >= 2 {
            let owner = next.owner[target.0][target.1];
            if i as i32 != owner {
                collected[i] = true;
            }
        }
    }

    for i in 0..game.m {
        if collected[i] {
            continue;
        }
        let (x, y) = temp_pos[i];
        let owner = next.owner[x][y];
        if owner == -1 {
            next.owner[x][y] = i as i32;
            next.level[x][y] = 1;
        } else if owner == i as i32 {
            if next.level[x][y] < game.u {
                next.level[x][y] += 1;
            }
        } else {
            next.level[x][y] -= 1;
            if next.level[x][y] == 0 {
                next.owner[x][y] = i as i32;
                next.level[x][y] = 1;
            } else {
                collected[i] = true;
            }
        }
    }

    for i in 0..game.m {
        if collected[i] {
            temp_pos[i] = state.pos[i];
        }
    }
    next.pos = temp_pos;
    next
}

pub(crate) fn absolute_score(game: &Game, state: &State) -> f64 {
    let scores = calc_scores(game, state);
    let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
    let ratio = scores[0] as f64 / sa;
    1e5 * (1.0 + ratio).log2()
}

pub(crate) fn choose_predicted_ai_top2_moves(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> Vec<((usize, usize), (usize, usize), f64)> {
    let mut moves = Vec::with_capacity(game.m.saturating_sub(1));
    for ai_idx in 0..game.m.saturating_sub(1) {
        let player = ai_idx + 1;
        let cands = get_candidates(game, state, player);
        if cands.is_empty() {
            let cur = state.pos[player];
            moves.push((cur, cur, 1.0));
            continue;
        }
        let probs = blended_ai_probs(game, state, player, &models[ai_idx], &cands);
        let mut order: Vec<usize> = (0..cands.len()).collect();
        order.sort_by(|&a, &b| {
            probs[b]
                .partial_cmp(&probs[a])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let i1 = order[0];
        let i2 = if order.len() >= 2 { order[1] } else { order[0] };
        let p1 = probs[i1];
        let p2 = probs[i2];
        let conf = if i1 == i2 { 1.0 } else { p1 / (p1 + p2 + 1e-12) };
        moves.push((cands[i1], cands[i2], conf.clamp(0.5, 1.0)));
    }
    moves
}

pub(crate) fn uncertainty_risk(top2: &[((usize, usize), (usize, usize), f64)]) -> f64 {
    if top2.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for (_, _, conf) in top2 {
        sum += 1.0 - *conf;
    }
    (sum / top2.len() as f64).clamp(0.0, 0.5)
}

pub(crate) fn build_secondary_ai_moves(
    scores: &[i64],
    top2: &[((usize, usize), (usize, usize), f64)],
    switch_cap: usize,
) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    if top2.is_empty() {
        return moves;
    }
    let s0 = scores.first().copied().unwrap_or(1).max(1) as f64;
    let mut ranked: Vec<(f64, usize)> = Vec::new();
    for (ai_idx, (p1, p2, conf)) in top2.iter().enumerate() {
        if p1 == p2 {
            continue;
        }
        let player = ai_idx + 1;
        let threat_ratio = (scores[player] as f64 / s0).clamp(0.2, 3.0);
        let threat = (1.0 - *conf) * threat_ratio;
        ranked.push((threat, ai_idx));
    }
    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let cap = switch_cap.max(1).min(ranked.len());
    for (_, i) in ranked.into_iter().take(cap) {
        moves[i] = top2[i].1;
    }
    moves
}

pub(crate) fn sample_index(probs: &[f64], rng: &mut FastRng) -> usize {
    if probs.is_empty() {
        return 0;
    }
    let r = rng.next_f64();
    let mut acc = 0.0;
    for (i, &p) in probs.iter().enumerate() {
        acc += p.max(0.0);
        if r <= acc {
            return i;
        }
    }
    probs.len() - 1
}

pub(crate) fn build_ai_candidates_and_probs(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> Vec<(Vec<(usize, usize)>, Vec<f64>)> {
    let mut all = Vec::with_capacity(game.m.saturating_sub(1));
    for ai_idx in 0..game.m.saturating_sub(1) {
        let player = ai_idx + 1;
        let cands = get_candidates(game, state, player);
        if cands.is_empty() {
            all.push((vec![state.pos[player]], vec![1.0]));
            continue;
        }
        let probs = blended_ai_probs(game, state, player, &models[ai_idx], &cands);
        all.push((cands, probs));
    }
    all
}

pub(crate) fn frontier_potential(game: &Game, state: &State) -> f64 {
    let mut frontier = 0.0_f64;
    let mut growth = 0.0_f64;
    let mut vulnerability = 0.0_f64;
    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for x in 0..game.n {
        for y in 0..game.n {
            if state.owner[x][y] != 0 {
                continue;
            }
            let v = game.v[x][y] as f64;
            let lv = state.level[x][y] as f64;
            if state.level[x][y] < game.u {
                growth += v * (game.u - state.level[x][y]) as f64 / game.u as f64;
            }
            for (dx, dy) in DIRS {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if !in_bounds(game.n, nx, ny) {
                    continue;
                }
                let ux = nx as usize;
                let uy = ny as usize;
                let nv = game.v[ux][uy] as f64;
                let owner = state.owner[ux][uy];
                if owner == -1 {
                    frontier += 1.00 * nv;
                } else if owner > 0 {
                    if state.level[ux][uy] == 1 {
                        frontier += 0.85 * nv;
                    } else {
                        frontier += 0.35 * nv / state.level[ux][uy] as f64;
                    }
                } else if state.level[ux][uy] == 1 && lv == 1.0 {
                    vulnerability += 0.45 * v;
                }
            }
        }
    }
    0.022 * frontier + 0.090 * growth - 0.060 * vulnerability
}

pub(crate) fn strategic_score(game: &Game, state: &State) -> f64 {
    absolute_score(game, state) + frontier_potential(game, state)
}

pub(crate) fn pessimism_weight(game: &Game, uncertainty: f64) -> f64 {
    if uncertainty < 0.08 {
        return 0.0;
    }
    let m_factor = ((game.m as f64 - 2.0) / 6.0).clamp(0.0, 1.0);
    (0.05 + 0.22 * uncertainty + 0.10 * m_factor).clamp(0.05, 0.32)
}

pub(crate) fn evaluate_local_move(
    game: &Game,
    state: &State,
    cand: (usize, usize),
    scores: &[i64],
    s0: f64,
    max_ai_i64: i64,
    phase: f64,
    conflict_map: &[Vec<f64>],
    cur: (usize, usize),
    is_leader: &[bool],
) -> f64 {
    let (x, y) = cand;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;
    let max_ai = max_ai_i64 as f64;
    let mut score = 0.0_f64;

    if owner == -1 {
        score += value;
        score += (1.0 - phase) * 0.52 * value;
    } else if owner == 0 {
        if level < game.u {
            score += 0.90 * value;
            score += 0.18 * value * (game.u - level) as f64 / game.u as f64;
        } else if (x, y) == cur {
            score -= 0.15 * value;
        } else {
            score -= 0.05 * value;
        }
    } else {
        let opp = owner as usize;
        let threat = ((scores[opp] as f64 - s0).max(0.0)) / max_ai;
        let m5_focus = (1.0 - ((game.m as f64 - 5.0).abs() / 2.0)).clamp(0.0, 1.0);
        if level == 1 {
            score += (1.25 + 0.85 * threat) * value;
            if is_leader[opp] {
                score += 0.45 * phase * value;
                score += (0.10 + 0.20 * phase) * m5_focus * (0.5 + threat) * value;
            }
        } else {
            score += (0.32 + 0.45 * threat) * value / level as f64;
            score -= 0.11 * value;
            if is_leader[opp] {
                score += 0.20 * phase * value / level as f64;
            }
        }
    }

    let next_pos = if owner > 0 && level >= 2 { cur } else { (x, y) };
    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in DIRS {
        let nx = next_pos.0 as isize + dx;
        let ny = next_pos.1 as isize + dy;
        if in_bounds(game.n, nx, ny) {
            let ux = nx as usize;
            let uy = ny as usize;
            let vv = game.v[ux][uy] as f64;
            if state.owner[ux][uy] != 0 {
                score += 0.07 * vv;
            } else if state.level[ux][uy] < game.u {
                score += 0.03 * vv;
            }
        }
    }

    let p_any = 1.0 - (-conflict_map[x][y]).exp();
    let multi_factor = ((game.m as f64 - 2.0) / 6.0).clamp(0.0, 1.0);
    let risk_scale = 1.0 + 0.35 * multi_factor + 0.20 * phase;
    if owner == -1 {
        score -= 0.75 * risk_scale * p_any * value;
    } else if owner == 0 {
        score += 0.08 * p_any * value / risk_scale;
    } else if level == 1 {
        score -= 0.30 * risk_scale * p_any * value;
    } else {
        score -= 0.18 * risk_scale * p_any * value;
    }

    score + value * 1e-6 - (x as f64 * 31.0 + y as f64) * 1e-9
}

fn update_model_for_player(
    game: &Game,
    state_before: &State,
    player: usize,
    observed: (usize, usize),
    model: &mut AiModel,
) {
    let cands = get_candidates(game, state_before, player);
    if cands.is_empty() {
        return;
    }

    let obs_idx = match cands.iter().position(|&x| x == observed) {
        Some(v) => v,
        None => return,
    };

    let mut est_scores = vec![0.0_f64; cands.len()];
    let mut feats = vec![[0.0_f64; 4]; cands.len()];
    for (i, &cand) in cands.iter().enumerate() {
        let f = ai_features(game, state_before, player, cand);
        feats[i] = f;
        est_scores[i] = dot4(&model.w, &f);
    }

    let max_score = est_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let tol = 1e-9 * max_score.abs().max(1.0);
    let best_set: Vec<usize> = (0..cands.len())
        .filter(|&i| est_scores[i] >= max_score - tol)
        .collect();
    let pred_idx = best_set.first().copied().unwrap_or(0);

    let informative = best_set.len() < cands.len();
    if informative {
        model.seen += 1;
        let matched = best_set.contains(&obs_idx);
        if !matched {
            model.mismatch += 1;
        }

        let raw_eps = model.mismatch as f64 / model.seen.max(1) as f64;
        model.eps_est = (0.70 * model.eps_est + 0.30 * raw_eps).clamp(0.05, 0.60);

        if !matched {
            for k in 0..4 {
                let diff = (feats[obs_idx][k] - feats[pred_idx][k]) / 1000.0;
                model.w[k] = (model.w[k] + 0.12 * diff).clamp(0.10, 2.00);
            }
        }
    }

    for k in 0..4 {
        model.w[k] = 0.995 * model.w[k] + 0.005 * 0.64;
    }
}

fn update_models(
    game: &Game,
    state_before: &State,
    selected: &[(usize, usize)],
    models: &mut [AiModel],
) {
    for ai_idx in 0..models.len() {
        let player = ai_idx + 1;
        update_model_for_player(game, state_before, player, selected[player], &mut models[ai_idx]);
    }
}

fn read_initial<R: BufRead>(sc: &mut Scanner<R>) -> Option<(Game, State)> {
    let n = sc.next::<usize>()?;
    let m = sc.next::<usize>()?;
    let t = sc.next::<usize>()?;
    let u = sc.next::<usize>()?;

    let mut v = vec![vec![0_i64; n]; n];
    for row in &mut v {
        for val in row.iter_mut() {
            *val = sc.next::<i64>()?;
        }
    }

    let mut pos = vec![(0_usize, 0_usize); m];
    for p in &mut pos {
        let x = sc.next::<usize>()?;
        let y = sc.next::<usize>()?;
        *p = (x, y);
    }

    let mut owner = vec![vec![-1_i32; n]; n];
    let mut level = vec![vec![0_usize; n]; n];
    for (i, &(x, y)) in pos.iter().enumerate() {
        owner[x][y] = i as i32;
        level[x][y] = 1;
    }

    let game = Game { n, m, t, u, v };
    let state = State {
        pos,
        owner,
        level,
        turn: 0,
    };
    Some((game, state))
}

fn read_feedback<R: BufRead>(
    sc: &mut Scanner<R>,
    game: &Game,
    state: &mut State,
) -> Option<Vec<(usize, usize)>> {
    let mut selected = vec![(0_usize, 0_usize); game.m];
    for s in &mut selected {
        let x = sc.next::<usize>()?;
        let y = sc.next::<usize>()?;
        *s = (x, y);
    }

    for p in 0..game.m {
        let x = sc.next::<usize>()?;
        let y = sc.next::<usize>()?;
        state.pos[p] = (x, y);
    }
    for i in 0..game.n {
        for j in 0..game.n {
            state.owner[i][j] = sc.next::<i32>()?;
        }
    }
    for i in 0..game.n {
        for j in 0..game.n {
            state.level[i][j] = sc.next::<usize>()?;
        }
    }
    state.turn += 1;
    Some(selected)
}

pub fn run_with_strategy(strategy: StrategyMode) {
    let stdin = io::stdin();
    let mut sc = Scanner::new(BufReader::new(stdin.lock()));
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let (game, mut state) = match read_initial(&mut sc) {
        Some(v) => v,
        None => return,
    };

    let mut models = vec![AiModel::new(); game.m.saturating_sub(1)];

    for _ in 0..game.t {
        let prev_state = state.clone();
        let (x, y) = choose_move(&game, &prev_state, &models, strategy);

        if writeln!(out, "{} {}", x, y).is_err() {
            return;
        }
        if out.flush().is_err() {
            return;
        }

        let selected = match read_feedback(&mut sc, &game, &mut state) {
            Some(v) => v,
            None => return,
        };
        update_models(&game, &prev_state, &selected, &mut models);
    }
}
