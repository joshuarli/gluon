# Testing

Parity is observable behavior, not text. A differential fixture passes when,
against the reference (Meson 1.12.1 / Ninja 1.13.2), it matches: output
artifacts (bytes, or symbols/install names for linked binaries where
timestamps/paths differ), the command set after normalizing paths and
argument order where order is not semantic, no-op and touch-rebuild
behavior (which edges rerun), and exit codes. Normalization rules live in
`tests/differential/` and are tested. Upstream Meson 1.12.1 is the oracle;
Muon is informational.

The expected-fail ledger (`tests/differential/expected-fail`) lists
fixtures known to diverge, one per line with a one-line cause. The harness
fails on any unlisted mismatch and on any listed fixture that now passes.
The ledger is empty when the campaign closed.

Gates, narrowest first:

- `cargo test --workspace` — unit/integration suites for all crates.
- `tests/differential/run.sh` — the fixture corpus; exit 0 only with the
  ledger honored.
- `tests/upstream/musl.sh` — static-musl bins via Docker (pinned
  `rust:1.98`, `--platform linux/amd64`, `x86_64-unknown-linux-musl`) that
  run `--version` in the container. CI builds the same target directly with
  `musl-tools` (see [decisions.md](decisions.md)).
- `tests/benchmarks/run.sh` — cold Meson setup, reconfigure, Ninja no-op at
  1K/10K/100K edges, one-source/one-header incremental rebuild; CPU/wall/
  peak RSS per leg, interleaved baseline and candidate, median over
  repeats. Time-based gates run only when `perf-gate-ready` exits 0, and a
  second pass with `-1` right after; a deferred gate never counts as a
  pass. The close gate is "no slower than the references beyond the
  observed spread" (CPU-time median); a regression beyond the spread, or a
  deferred gate, blocks the work.

Qualification (`~/d/play`, read-only): drive libplacebo 7.387.0 with the
Metal patch and SPIRV-Cross vulkan-sdk 1.4.363.0 through gluon `meson` +
`ninja` per play's `scripts/build-deps.sh` Meson invocations, then link
play's player crates against the outputs with `CARGO_TARGET_DIR` under this
repo's `.work/tmp`.
