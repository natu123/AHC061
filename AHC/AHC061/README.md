# AHC061 - THIRD Programming Contest 2026

## Overview
My solutions for [AHC061 (THIRD Programming Contest 2026)](https://atcoder.jp/contests/ahc061) - a multi-player territory game on a 10x10 board.

This repository contains Phase 1 of development (solvers x01-x81). Phase 2 continues in [AHC061-2](https://github.com/natu123/AHC061-2).

## Problem
- 10x10 board, M=2..8 players, T=100 turns, U=1..5 level cap
- Control player 0, maximize S0/SA ratio
- Score: round(10^5 * log2(1 + S0 / SA))

## Structure
```
solver/src/bin/    # All solver implementations (x01-x81)
solver/src/lib.rs  # Shared library
docs/              # Game rules, experiment logs, solver specs
submissions/       # Submission source files
AGENTS.md          # AI agent instructions (used with Claude Code)
CLAUDE.md          # Claude Code project config
```

## Solver Lineage (Phase 1)
- x01: Beam Search Pessimistic
- x04: Macro Route (became foundation for many variants)
- x47: Macro Route Pressure Dual Guard (Phase 1 Champion)
- x64-x66: Portfolio Mixer series
- x67-x70: Gear Shift Hybrid series
- Full registry: `docs/solver_specs_built/`

## Tools
Download the official AHC061 tools from [AtCoder](https://atcoder.jp/contests/ahc061) and place them in `N52XwIfp_windows/`.

## Built with
- Rust (edition 2021)
- [Claude Code](https://claude.ai/claude-code) (Opus 4.6) as AI pair programming partner
