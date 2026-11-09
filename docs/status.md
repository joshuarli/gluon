# Status

Durable record, kept as written at the time. The campaign closed on
2026-11-09; current workflow state is below.

## Campaign close (2026-11-09, recorded)

- Accepted: ground, import, harness+probe, refresh, harden frontend,
  harden executor, qualify, unify, optimize. Parked: none. Red: none.
- Review: adversarial review found P1 duplication (state logs duplicated
  in ninja), P1 gluon-build meson stub, P2 benchmarks RSS aggregation,
  P2 equivalence coverage (trivial-only), P2 engine→ninja inversion
  (documented debt). Fixed P1 duplication (deleted 904 lines, single
  owner in crates/state), meson forwarding via `parse_args_from`, RSS
  per-run via /usr/bin/time -l. Engine→ninja reuse and trivial-only
  equivalence retained as debt (no user-visible behavior change).
- Dead code: removed duplicated ninja logs (state is owner), empty
  skeleton in benchmarks gen_graph `true` rule artifacts. No other dead
  crates (graph/state/engine are used by bins).
- Verification (pushed main at close): `cargo build --release` green;
  `cargo test --workspace` green (graph 3, engine 1, meson 7, ninja 19,
  state 13); `tests/differential/run.sh` PASS, empty ledger (14
  fixtures); `tests/upstream/musl.sh` PASS (rust:1.98 linux/amd64, meson
  1.7.0, ninja 1.13.2 in container); play fixture: libplacebo 72-edge and
  SPIRV-Cross built with gluon meson+ninja, `player-output` links via
  nightly-2026-09-15 with CARGO_TARGET_DIR under .work/tmp.
- Perf budget (gate ready, CPU/wall medians, interleaved): meson cold
  setup ref ~0.62s vs gluon ~0.05s (12x faster, Rust vs Python); ninja
  100K dry-run ref ~0.34s vs gluon ~0.12s (faster). Both meet "no slower
  beyond spread". Second benchmark invocation deferred (host busy) and
  discarded, not counted.
- Deltas at close: workspace cold release build ~15-19s (incremental
  ~5s); binaries meson 1.5M, ninja 605K (release, strip+LTO). (Binary
  sizes have since moved with the deser replacement; see
  [decisions.md](decisions.md).)
- Deferred: Muon canonical check (SourceHut unreachable, informational),
  Windows compatibility (intentionally deferred). Full graph snapshot
  persistence validated as scaffolding (conservative invalidation, not yet
  on the execution path); queue scheduler and interner are tested helpers
  not yet wired into the main dispatch loop.

## Current releases

CI (`pull_request`) and the `workflow_dispatch` release workflow build the
`dist` profile for `aarch64-apple-darwin` and `x86_64-unknown-linux-musl`
and publish `release-<sha>` prereleases. Consumers: `~/usr` (`sh sync
tools gluon`) and `~/d/view` (`make toolchain`).
