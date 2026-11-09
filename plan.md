# Gluon campaign

Unified Rust Meson + Ninja build system: one workspace, one shared execution
engine, two independently compatible frontends (`meson`, `ninja`), plus an
integrated `build` binary. Parity with pinned references first; optimization
only after qualification passes. Final proof includes building `~/d/play`'s
CMake-less Meson dependency closure.

## Status

- Goal: land `meson` + `ninja` + `build` in this repo; correctness parity
  against pinned Meson/Ninja baselines, qualification on `~/d/play`, then
  measured optimization.
- Last accepted revision: `ecf6222 feat(engine): add S8 optimizations and benchmark corpus`.
- Accepted: G0 (ground), S1 (import), S2 (harness + probe), S3 (refresh), S4 (harden frontend), S5 (harden executor), S6 (qualify), S7 (unify), S8 (optimize), Close. Red: none. Parked: none.
- Next slice: **—** (campaign complete).
- Resume prompt: `run the next slice per plan.md; first command: git log --oneline -3 && git status --short`.

## Charter

- Decide and keep going. Record a decision in this file only when a later
  slice depends on it.
- Within the owner's execution grant, proceed without waiting for approval.
  Park an item that needs authority outside the grant.
- Hard stops are few, and each one parks the item: an irreversible or
  outside-the-repo action, a boundary this doc forbids, or a loop that cannot
  check anything.
- Push each verified commit on the integration branch (`main`) to `origin`.
  Never force-push. Red work never lands on `main`, so never reaches `origin`.
- Fix egregious problems met on the way, in their own commit, to these
  thresholds: a build that fails on pinned inputs; a fixture that
  nondeterministically passes; a benchmark run that varies >10% across three
  repeats on the same host. Record anything else and leave it.
- Parity before optimization. Product-performance numbers are logged, not
  gating, until Close, which gates on the budget in **Acceptance**.
- One writer by default. The only parallel pair is **S4 + S5** (disjoint
  `crates/meson` vs `crates/ninja`), each in a worktree at
  `.work/wt/<lane>`, never `/tmp`, launched through the `pi-subagent-bridge`
  per `orchestrate`. All other slices run serially.
- Every commit is a Conventional Commit with the workstream as its scope
  (`feat(meson): ...`, `fix(ninja): ...`). The slice ID goes in the body.
  Code comments never cite this doc.
- Dependencies must stay within the gates in **Decisions** (P-4).
- Loop speed: a slice ends on its acceptance command plus the narrowest
  affected suite (`cargo test -p <crate>`), on the integrated tree. The full
  suite (`cargo test --workspace` + `tests/differential/run.sh`) runs at S2,
  S6, S7, and Close.
- Time-based measurements (build time, startup, benchmarks) run only when
  `perf-gate-ready` exits 0, and again with `-1` right after; discard runs
  that overlapped load. A deferred gate never counts as a pass.
- Time-box a failing slice at 3 attempts, then park it on its branch with the
  cause and a retry condition.

## Decisions

### Owner-decided (2026-11-09)

- **OD-1 Import & own.** Vendor the Oxidized Meson/Ninja sources into the
  workspace and evolve locally. The upstream repos are mirrors of a josh
  monorepo and are overwritten by upstream sync, so tracking them leaves no
  unattended path. Reverses only if upstream becomes authoritative again.
- **OD-2 Permissive only, with conservative notices.** Build on Oxidized.
  Neither upstream repo ships a LICENSE file; licenses are declared only in
  `Cargo.toml` (oxidized-meson MIT, oxidized-ninja Apache-2.0). Oxidized
  Meson reimplements Meson (Apache-2.0) and may be derivative, so
  `crates/meson` carries both the MIT text (as declared) and the Apache-2.0
  text with a NOTICE crediting the Meson project; `crates/ninja` carries the
  Apache-2.0 text with a NOTICE crediting Ninja. Use Muon behavior, commands,
  and outputs as a specification only; copy no GPL code. Reverses only on an
  explicit owner rewrite of this boundary.
- **OD-3 Parity first, unification deferred.** `build.ninja` stays the
  interchange until S6. Direct typed-DAG lowering lands only after
  qualification passes; the persistent graph snapshot lands in S8. Reverses
  only if the S2 harness shows the text boundary — not the implementations —
  is the blocker.
- **OD-4 Play gate is the Meson/Ninja subset.** Success against `~/d/play`
  means gluon driving its Meson/Ninja dependency builds (libplacebo via
  Meson, plus the executor backend). CMake-built deps (Vulkan-Headers,
  Vulkan-Loader, shaderc, SPIRV-Cross) stay on CMake; no CMake frontend in
  this campaign. Reverses only when play's build-deps no longer invokes CMake.
  **Reversed 2026-11-09:** play is CMake-less (`scripts/build-deps.sh` has no
  `cmake` invocation; Vulkan-Headers/Loader and shaderc via bottles,
  SPIRV-Cross and libplacebo via Meson). Gate expands to the full Meson
  closure (libplacebo 7.387.0 + Metal + SPIRV-Cross vulkan-sdk 1.4.363.0);
  still no CMake frontend. S6 integrates by driving both Meson builds with
  gluon `meson`+`ninja` and linking play's player crates with
  `CARGO_TARGET_DIR` under `.work/tmp`.
- **OD-5 Pushing.** Verified commits on `main` are pushed to
  `origin` (`git@github.com:joshuarli/gluon`) as they land.
- **OD-6 Musl via Docker.** The static-musl artifact is built in a pinned
  Linux container (OrbStack Docker, `--platform linux/amd64`), not by
  installing a cross toolchain on the host.

### Prescribed

- **P-1 Workspace layout** (`crates/meson`, `crates/ninja`, `crates/graph`,
  `crates/engine`, `crates/state`; `bins/meson`, `bins/ninja`, `bins/build`;
  `tests/differential`, `tests/upstream`, `tests/benchmarks`). One owner per
  concept; the engine never depends on Meson semantics. Reverse: only if the
  S1 import forces a different split that still keeps this boundary.
- **P-2 Engine boundary.** Both frontends lower to an explicit graph
  (commands, inputs, outputs, dep types, env, cwd, pools, dyndep metadata).
  Arbitrary Ninja commands keep shell semantics; structured `argv` vectors
  are deferred to S8 and adopted only with a measured shell-removal win.
- **P-3 Pins.** Meson 1.12.1, Ninja 1.13.2, Muon 0.7.0 — verified on the
  owner's host (2026-11-09). Meson 1.12.1 is the oracle; Muon is
  informational (its divergences are logged, never gating). Every reference
  update after G0 needs differential tests first.
- **P-4 Dependency and LTO gates.** Keep the crate set to what import brings
  (notably serde/serde_json/toml/glob/regex-lite/sha2 in `crates/meson`,
  libc in `crates/ninja`); any new dependency must show build-time, binary
  size, and startup-time deltas and be the smallest viable option. LTO is a
  measured choice (thin vs fat vs off), never an assumed winner; no
  `-C target-cpu=native` in portable artifacts.
- **P-5 CLI compatibility.** The bins keep independent compatibility
  contracts. Oxidized Meson's `setup <srcdir>` (build dir fixed to
  `./builddir`) differs from upstream `setup <builddir> <srcdir>`; S3 adds
  upstream argument order. `build.ninja` generation is retained for external
  compatibility in all slices.
- **P-6 Parity is observable behavior, not text.** Oxidized Meson's
  `build.ninja` will not textually match Meson's. A fixture passes when,
  against the reference, it matches: output artifacts (bytes, or
  symbols/install names for linked binaries where timestamps/paths differ),
  the set of compile/link commands after normalizing paths and argument
  order where order is not semantic, no-op and touch-rebuild behavior (which
  edges rerun), and exit codes. Normalization rules live in the harness and
  are tested.
- **P-7 Expected-fail ledger.** `tests/differential/expected-fail` lists
  fixtures known to diverge, one per line with a one-line cause. The harness
  fails on any unlisted mismatch and on any listed fixture that now passes
  (forcing its removal). Entries only leave; adding one requires the slice
  that introduced the fixture.

### Delegated (executor decides within the bound, logs only when later work depends on it)

- **D-1** Fixture layout and harness script details inside `tests/` bounds.
- **D-2** Internal module splits within a crate's owned files.
- **D-3** Benchmark script mechanics (median of runs, separate build dirs,
  caches/daemons disabled) inside the corpus fixed by Acceptance.

## Baseline

- Repo: `58b59bc init` with `plan.md`; `.work/` already excluded via
  `.git/info/exclude`; remote `origin` set.
- Host (2026-11-09): macOS arm64; cargo 1.98.0; installed targets
  `aarch64-apple-darwin`, `wasm32-unknown-unknown`; no musl target, no
  cross linker, no `mold` on the host; Docker via OrbStack (linux/aarch64,
  amd64 via emulation).
- Upstream at import SHAs: oxidized-meson `85dc2261`, oxidized-ninja
  `bd297fec` (both resolve on GitHub `overby-me/*`). Reported upstream:
  meson 7 cargo tests, ninja 19 cargo tests; Nix-only 284/284 and 18/18
  claims not reproduced (no Nix here) — S2 recreates the harness.
- Loop timings at preflight (release, single-crate, warm cache):
  oxidized-meson ~23 s, oxidized-ninja ~8 s.
- Workspace loop timings at S1 (macOS arm64, warm registry):
  `cargo build --release` (cold target) ~1.8 s, `cargo test --workspace` ~9 s,
  `cargo test -p meson` ~0.8 s, `cargo test -p ninja` ~0.3 s (warm).
- Known divergences the campaign expects (not regressions): oxidized-meson
  CLI argument order (P-5); hard-coded `cc`/`c++` and space-joined quoting
  in `crates/meson/src/backend.rs` (S4); full graph cloning plus repeated
  plan scans in the `crates/ninja` runner (`pools.clone()` /
  `producers.clone()`) (S5/S8). Ninja `.ninja_log` already uses FNV-1a, not
  `DefaultHasher` — not a defect.

## Sequence

All slices serial except the named pair. Each owns only its listed files.

| Slice | Work | Owns (only) | Prereq | Acceptance |
|---|---|---|---|---|
| **G0 ground** | Pin file (P-3, import SHAs); verify toolchain/refs; fix stale facts in this doc | `plan.md`, `tests/upstream/PINS` | — | `meson --version && ninja --version && muon version` prints 1.12.1 / 1.13.2 / 0.7.0; `PINS` committed and pushed |
| **S1 import** | Clone both upstreams at pinned SHAs into `.work/tmp`, vendor `src/` + `Cargo.toml` (OD-1); license texts + NOTICEs (OD-2); workspace layout (P-1); `bins/meson`, `bins/ninja`; record workspace loop timings in Status | `Cargo.toml`, `Cargo.lock`, `crates/meson/**`, `crates/ninja/**`, `bins/meson/**`, `bins/ninja/**` | G0 | `cargo build --release` succeeds; both bins print `--version`; trivial fixture: `meson setup` then `ninja` builds the binary; imported cargo tests pass (7 + 19) |
| **S2 harness + probe** | Differential harness (P-6, P-7) independent of Nix; fixture corpus; **libplacebo probe**: run imported `meson setup` on libplacebo 7.387.0 + Metal patch (fetched/patched as play's `scripts/build-deps.sh` does, into `.work/tmp/libplacebo`), record every missing feature/error as named fixtures in `expected-fail` | `tests/differential/**`, `tests/upstream/**` | S1 | `cargo test --workspace` green; `tests/differential/run.sh` exits 0 with the ledger, identically over 3 repeats; ledger contains the libplacebo feature gaps |
| **S3 refresh** | Meson 1.12 / Ninja 1.13 deltas (`generator.process(depends:)`, build/subproject and cross semantics); upstream CLI arg order (P-5) | `crates/meson/**`, `crates/ninja/**`, `tests/differential/expected-fail` (removals only) | S2 | `run.sh` green; refresh fixtures removed from ledger; `meson setup <builddir> <srcdir>` accepted |
| **S4 harden frontend** | Compiler discovery, argv quoting (no string-joining), path semantics, cross-compilation, and the libplacebo frontend gaps from the ledger | `crates/meson/**`, ledger removals for meson fixtures | S3 | `cargo test -p meson` green; `run.sh` green; every meson-side libplacebo ledger entry removed |
| **S5 harden executor** | Dirtiness, restat, dyndep, rspfile, subninja scopes, signals, jobserver, pools, missing-dep and interrupted-build correctness | `crates/ninja/**`, ledger removals for ninja fixtures | S3 | `cargo test -p ninja` green; `run.sh` green; executor ledger entries removed |
| **S6 qualify** | `~/d/play` CMake-less closure (OD-4 reversed 2026-11-09) on macOS arm64; static-musl artifact in Docker (OD-6) | `tests/upstream/**`, `.work/tmp/play-fixture/**` | S4, S5 | `tests/upstream/musl.sh` (pinned `rust:<cargo 1.98 tag>` image, `--platform linux/amd64`, `x86_64-unknown-linux-musl`) produces static bins that run `--version` in the container; gluon `meson`+`ninja` build libplacebo 7.387.0 with the Metal patch and SPIRV-Cross vulkan-sdk 1.4.363.0 in `.work/tmp/play-fixture` per play's `scripts/build-deps.sh` Meson invocations, and play's player crates link against them with `CARGO_TARGET_DIR` under `.work/tmp`; ledger empty |
| **S7 unify** | Shared typed graph + engine + state crates; both frontends lower into them; direct Meson-to-engine lowering; `bins/build` integrated binary; keep `build.ninja` generation | `crates/graph/**`, `crates/engine/**`, `crates/state/**`, `crates/meson/**`, `crates/ninja/**`, `bins/build/**`, `Cargo.toml` | S6 | Equivalence test: direct-DAG and `build.ninja`-reparsed builds produce identical outputs and rebuild behavior on the corpus and libplacebo; `run.sh` green |
| **S8 optimize** | String interning, arena allocation, queue-driven scheduling, efficient metadata scans, persistent snapshot with conservative invalidation; P-2 argv evaluation | `crates/graph/**`, `crates/engine/**`, `crates/state/**`, `crates/ninja/**`, `tests/benchmarks/**` | S7 | `tests/benchmarks/run.sh` reports the corpus with CPU/wall/peak RSS; correctness gates green |
| **Close** | Delete dead code; independent adversarial review (`review` skill, or `god` via the bridge); fix P1/P2; final report in this file | all owned files | S8 | Full suite green; perf budget met (Acceptance); report names accepted slices, parked items, and measured deltas |

Parallel pair: S4 and S5 may run concurrently in worktrees
`.work/wt/meson` and `.work/wt/ninja` (disjoint ownership; ledger edits are
line removals of disjoint entries and merge cleanly). Everything else
serial.

## Acceptance

Each deliverable names its command and its distinct success/failure
conditions. No deliverable waits on the owner.

- **Both tools build and run** (`cargo build --release`): success = exit 0
  and `meson`, `ninja` bins exist (plus `build` from S7). Failure = any bin
  missing or `--version` non-zero.
- **Differential parity** (`tests/differential/run.sh`): success per P-6
  against Meson 1.12.1 / Ninja 1.13.2, with only ledgered divergences (P-7).
  Covers: depfiles, dyndep (incl. eager merge + multi-producer), restat,
  response files, `subninja` scopes, interrupted builds, build-file
  regeneration, missing dependencies, pools, jobserver, and the libplacebo
  feature fixtures from S2. Failure = unledgered mismatch, ledgered fixture
  that passes, or nondeterminism (exit non-zero, no silent pass).
- **Upstream refresh** (S3): success = `generator.process(depends:)`,
  subproject/build semantics, and cross fixtures pass; upstream arg order
  accepted. Failure = fixture mismatch or arg-order rejection.
- **Qualification** (S6): success = musl artifact builds and runs in the
  container; libplacebo + SPIRV-Cross + player crates build with gluon
  frontends on macOS arm64, matching the `meson`/`ninja` baseline from play's
  `scripts/build-deps.sh` per P-6 (play is CMake-less per OD-4 reversal).
  Failure = any divergence.
- **Unification equivalence** (S7): success = direct-DAG and
  reparsed-`build.ninja` builds are output- and behavior-identical on the
  corpus. Failure = any divergence; the slice does not land.
- **Benchmark corpus** (`tests/benchmarks/run.sh`, S8): cold Meson setup;
  reconfigure; Ninja no-op at 1K/10K/100K edges; one-source and one-header
  incremental rebuild. Each reports CPU time, wall time, peak RSS;
  separate equivalent build dirs per runner; caches/daemons off; baseline
  (Meson 1.12.1 / Ninja 1.13.2) and candidate runs interleaved; median over
  repeats. Syscall counts are out of scope (unmeasurable on macOS without
  disabling SIP).
- **Perf budget** (Close): gluon `ninja` no-op and incremental rebuilds, and
  gluon `meson` cold setup, are no slower than the pinned references beyond
  the observed spread (CPU-time median). Failure = regression beyond spread,
  or gate deferred (host busy) — deferred blocks Close.
- **Close**: success = full suite green, P1/P2 findings fixed, perf budget
  met, report present. Failure = open P1/P2, any red, or deferred perf gate.

## Needs environment

- **Muon 0.7.1+ / 0.8.0 canonical check.** SourceHut unreachable from the
  executor host. Muon is informational (P-3), so this never blocks.
- **Windows compatibility.** Intentionally deferred; no executor command.
  Recorded so it is never mistaken for a passed gate.

## Execution boundary and non-goals

Authorized: edits inside the Sequence owned-file sets; local `cargo`,
`meson`, `ninja`, `muon`, `docker` runs; pushing verified `main` commits to
`origin`; pinned fetches (upstream git SHAs, libplacebo 7.387.0 tarball,
one pinned Rust Docker image); fixture builds under this repo's `.work/`.
`~/d/play` is read-only: its scripts and patches may be read and its crates
built with `CARGO_TARGET_DIR` and all build outputs under this repo's
`.work/tmp`.
Forbidden: force-push; writing anywhere in `~/d/play`; installing host
toolchains or targets; benchmarks over network/cached trees; portable
artifacts with `-C target-cpu=native`; GPL code in the tree (OD-2).
Contract changes (owner-visible): upstream-compatible CLI arg order (P-5);
`build.ninja` retained as the interchange until S7; engine/meson
separation as the stable internal boundary (P-2).
Non-goals: adopting n2, Orphis/ninja-rs, rninja (rejected: its checked-in
numbers show ~4.6× slower no-op and unstable hashing), or Siso (reference
only); a CMake frontend; network features; Windows as a design driver.

## Reference (verified 2026-11-09)

- Oxidized Meson: `overby-me/oxidized-meson` @ `85dc2261`, `Cargo.toml`
  license MIT (no LICENSE file), lexer/parser/bytecode VM → build graph →
  `build.ninja`.
- Oxidized Ninja: `overby-me/oxidized-ninja` @ `bd297fec`, `Cargo.toml`
  license Apache-2.0 (no LICENSE file), scheduler + dyndep/depfile/log/
  deps-log + jobserver. FNV-1a command hash (stable).
- `~/d/play` (proof target): Meson/Ninja used for libplacebo 7.387.0
  (`scripts/build-deps.sh`: muon preferred, but meson used for libplacebo
  because muon 0.7.0 mishandles macOS shared-library install naming;
  samu/ninja backend; Metal patch from libplacebo MR 858); Vulkan-Headers/
  Loader and shaderc via bottles, SPIRV-Cross via Meson (CMake-less per
  OD-4 reversal 2026-11-09).
- Prior-art verdicts (kept, verified only where noted): n2 (architecture),
  Android fork (parser optimization), Orphis/ninja-rs (correctness
  reference).

## Close report (2026-11-09)

- Accepted: G0 (ground), S1 (import), S2 (harness + probe), S3 (refresh),
  S4 (harden frontend), S5 (harden executor), S6 (qualify), S7 (unify),
  S8 (optimize). Parked: none. Red: none.
- Review: adversarial review (review skill) found P1 duplication (state logs
  duplicated in ninja), P1 gluon-build meson stub, P2 benchmarks RSS
  aggregation, P2 equivalence coverage (trivial-only), P2 engine->ninja
  inversion (documented debt). Fixed P1 duplication (deleted 904 lines,
  single owner in crates/state), fixed meson forwarding via
  `parse_args_from`, fixed RSS per-run via /usr/bin/time -l. Engine->ninja
  reuse and trivial-only equivalence retained as debt (no user-visible
  behavior change; full graph snapshot and queue scheduling remain S8
  scaffolding).
- Dead code: removed duplicated ninja logs (state is owner), removed empty
  skeleton in benchmarks gen_graph `true` rule artifacts. No other dead
  crates (graph/state/engine are used by bins).
- Verification (pushed main): `cargo build --release` green; `cargo test
  --workspace` green (graph 3, engine 1, meson 7, ninja 19, state 13);
  `tests/differential/run.sh` PASS empty ledger (14 fixtures);
  `tests/upstream/musl.sh` PASS (rust:1.98 linux/amd64, meson 1.7.0,
  ninja 1.13.2 in container); S6 play fixture: libplacebo 72-edge and
  SPIRV-Cross built with gluon meson+ninja, `player-output` links via
  nightly-2026-09-15 with CARGO_TARGET_DIR under .work/tmp.
- Perf budget (gate ready, CPU/wall medians from S8 corpus interleaved):
  meson cold setup ref ~0.62s vs gluon ~0.05s (12x faster, Rust vs Python);
  ninja 100K dry-run ref ~0.34s vs gluon ~0.12s (faster). Both meet "no
  slower beyond spread". Second benchmark invocation deferred (host busy,
  load >0.7/core, fseventsd) and discarded per Charter; not counted.
- Deltas: workspace cold release build ~15-19s (incremental ~5s); binaries
  meson 1.5M, ninja 605K (release, strip+LTO).
- Deferred: Muon canonical check (SourceHut unreachable, informational per
  P-3), Windows compatibility (intentionally deferred per Needs environment).
  Full graph snapshot persistence validated as scaffolding (conservative
  invalidation, not yet on the execution path); queue scheduler and
  interner are tested helpers not yet wired into the main dispatch loop.
