# rust-2048-solver

A performance-focused 2048 solver in Rust, built around compact board representations, fast move generation, heuristic evaluation, and tree search.

The project explores how far a 2048 agent can be pushed by treating the game engine and search loop as systems code rather than a high-level prototype.

## Highlights

- Rust implementation of 2048 game state and search
- optimized 4-cell swipe path using packed integer operations
- MeanMax-style search with depth/deadline control
- transposition/search caching
- heuristic evaluation for board quality
- Criterion benchmarks for board operations and search

## Run

```bash
cargo run --release
```

Run the benchmark suite with:

```bash
cargo bench
```

## Project layout

- `src/game/` — game rules and board representation
- `src/bots/` — solver/search implementations and heuristics
- `src/accumulator/` — search result accumulation strategies
- `benches/` — Criterion benchmarks for board operations and search

## Why this exists

2048 is small enough that implementation details matter. Fast state transitions, compact representations, pruning/caching, and heuristic quality all directly affect how much of the search tree can be explored under a fixed move-time budget.

This repository is primarily an experimentation ground for that systems/search boundary.
