# Gluon campaign

Unified Rust Meson + Ninja build system: one workspace, one shared execution
engine, two independently compatible frontends (`meson`, `ninja`), plus an
integrated `build` binary. Parity with pinned references first; optimization
only after parity passes. Final proof includes building `~/d/play`'s
Meson/Ninja dependency subset.

## Status

- Goal: land `meson` + `ninja` + `build` in this repo; correctness parity
  against pinned Meson/Ninja/Muon baselines, then measured optimization, then
  qualification on `~/d/play`.
- Last accepted revision: none. Baseline commit pending (see Sequence G0).
- Accepted: none. Red: none. Parked: none.
- Next slice: **G0 (ground)** — baseline, pins, loop timings.
- Resume prompt: `run the next slice per chat.md; first command: git log --oneline -3 && cargo --version`.
  First command: `cargo build --release` (after S1) or `git status --short` (until S1).

## Charter

- Decide and keep going. Record a decision only when a later slice depends on it.
- Within the owner's execution grant, proceed without waiting for approval.
  Park an item that needs authority outside the grant.
- Hard stops are few, and each one parks the item: an irreversible or
  outside-the-repo action, a boundary this doc forbids, or a loop that cannot
  check anything. Pushing is always forbidden.
- Fix egregious problems met on the way, in their own commit, to these
  thresholds: a build that fails on pinned inputs; a fixture that
  nondeterministically passes; a benchmark run that varies >10% across three
  repeats on the same host. Record anything else and leave it.
- Parity before optimization. Until S6 passes, product-performance budgets
  are measured and logged, not gating, unless a slice states otherwise.
- One writer by default. The only parallel pair is **S4 + S5** (disjoint
  `crates/meson` vs `crates/ninja`), each in a worktree at
  `.work/wt/<lane>`, never `/tmp`, following `orchestrate`'s worktree
  lifecycle. All other slices run serially.
- Every commit is a Conventional Commit with the workstream as its scope
  (`feat(meson): ...`, `fix(ninja): ...`). The slice ID goes in the body.
  Code comments never cite this doc.
- Dependencies must stay within the gates in **Decisions** (Prescribed 4).
  Measure build time, binary size, and startup time when adding a dependency.
- Loop speed: a slice ends on its acceptance command plus the narrowest
  affected suite (`cargo test -p <crate>`), on the integrated tree. The full
  suite (`cargo test --workspace` + `tests/differential/run.sh`) runs at G0,
  S2, S6, and the close. Red work never lands on the integration branch.
- Time-box a failing slice at 3 attempts, then park it on its branch with the
  cause and a retry condition.

## Decisions

### Owner-decided (2026-11-09)

- **OD-1 Import & own.** Vendor the Oxidized Meson/Ninja sources into the
  workspace with license and provenance notes; evolve locally. The mirrors
  are read-only and overwritten by upstream sync, so tracking them leaves no
  unattended path. Reverses only if upstream becomes authoritative again.
- **OD-2 Permissive only.** Build on Oxidized (Meson MIT, Ninja Apache-2.0).
  Use Muon behavior, commands, and outputs as a specification; copy no GPL
  code. Keeps the result permissive. Reverses only on an explicit
  owner rewrite of this boundary.
- **OD-3 Parity first, unification deferred.** `build.ninja` stays the
  interchange until S6. Direct typed-DAG lowering and the persistent graph
  snapshot land only after imported parity passes. Reverses only if the S2
  harness shows the text boundary — not the implementations — is the blocker.
- **OD-4 Play gate is the Meson/Ninja subset.** Success against `~/d/play`
  means gluon driving its Meson/Ninja dependency builds (libplacebo via
  Meson, plus the executor backend). CMake-built deps (Vulkan-Headers,
  Vulkan-Loader, shaderc, SPIRV-Cross) stay on CMake until play itself drops
  CMake; no CMake frontend in this campaign. Reverses only when play's
  build-deps no longer invokes CMake.

### Prescribed

- **P-1 Workspace layout** (`crates/meson`, `crates/ninja`, `crates/graph`,
  `crates/engine`, `crates/state`; `bins/meson`, `ninja`, `build`;
  `tests/differential`, `tests/upstream`, `tests/benchmarks`). One owner per
  concept; the engine never depends on Meson semantics. Reverse: only if the
  S1 import forces a different split that still keeps this boundary.
- **P-2 Engine boundary.** Both frontends lower to an explicit graph
  (commands, inputs, outputs, dep types, env, cwd, pools, dyndep metadata).
  Arbitrary Ninja commands keep shell semantics; structured `argv` vectors
  are deferred to S7 and adopted only with a measured shell-removal win.
- **P-3 Pins.** Meson 1.12.1, Ninja 1.13.2, Muon 0.7.0 — the versions verified
  on the owner's host at preflight (2026-11-09). The draft's Muon 0.7.1 was
  not verifiable (canonical SourceHut unreachable from here) and is not
  assumed. Every reference update after G0 needs differential tests first.
- **P-4 Dependency and LTO gates.** Keep the crate set to what import brings
  (notably serde/serde_json/toml/glob/regex-lite/sha2 in `crates/meson`,
  libc in `crates/ninja`); any new dependency must show build-time, binary
  size, and startup-time deltas and be the smallest viable option. LTO is a
  measured choice (thin vs fat vs off), never an assumed winner; no
  `-C target-cpu=native` in portable artifacts.
- **P-5 CLI compatibility.** The bins keep independent compatibility
  contracts. Oxidized Meson's `setup <srcdir>` (build dir fixed to
  `./builddir`) differs from upstream `setup <builddir> <srcdir>` (verified
  at preflight); S3 adds a shim accepting the upstream argument order.
  `build.ninja` generation is retained for external compatibility in all
  slices.

### Delegated (executor decides within the bound, logs only when later work depends on it)

- **D-1** Fixture layout and harness script details inside `tests/` bounds
  (smallest reversible assumption; corpus is fixed by Acceptance).
- **D-2** Internal module splits within a crate's owned files.
- **D-3** Benchmark script mechanics (median of runs, separate build dirs,
  caches/daemons disabled) inside the corpus fixed by Acceptance.

## Baseline

- Repo: no commits yet; `chat.md` is the only file. G0 creates the initial
  commit and the `.work/` exclude entry.
- Loop timings measured at preflight (release, single-crate, warm cache):
  oxidized-meson ~23 s, oxidized-ninja ~8 s. Workspace numbers come from G0.
- Upstream cargo suites at import SHAs (meson `85dc226`,
  ninja `bd297fe`): meson 7 tests pass, ninja 19 tests pass.
- Verified at preflight: both crates build; oxidized-meson `setup` on a
  trivial fixture generates `build.ninja` with hard-coded `cc`/`c++`
  (`crates/meson/src/backend.rs:74,92,141,146` at import); oxidized-ninja
  builds that output. Upstream 284/284 and 18/18 claims are Nix-only
  (monorepo harness not in the mirrors; Nix is not installed here) and were
  **not** reproduced — S2 recreates the harness.
- Known divergences the campaign expects (not regressions): oxidized-meson
  CLI argument order (P-5); hard-coded compilers and space-joined quoting
  (S4); full graph cloning plus repeated plan scans in the scheduler
  (`crates/ninja` runner, `pools.clone()` / `producers.clone()` at import)
  (S5/S7); Ninja `.ninja_log` already uses FNV-1a, not `DefaultHasher` —
  not a defect.

## Sequence

All slices serial except the named pair. Each owns only its listed files.

| Slice | Work | Owns (only) | Prereq | Acceptance |
|---|---|---|---|---|
| **G0 ground** | Initial commit; pin file `tests/upstream/PINS` (P-3, import SHAs); verify toolchain/refs; record loop timings; confirm `.work/` excluded | `chat.md`, `tests/upstream/PINS`, `.git/info/exclude` | — | `meson --version && ninja --version && muon version` prints 1.12.1 / 1.13.2 / 0.7.0; `git log --oneline -1` exists |
| **S1 import** | Vendor both sources (OD-1, OD-2); workspace layout (P-1); provenance + licenses; both binaries run unchanged | `Cargo.toml`, `crates/meson/**`, `crates/ninja/**`, `bins/**` | G0 | `cargo build --release` succeeds; `bins/meson --version` and `bins/ninja --version` print; trivial fixture: `meson setup` then `ninja` builds the binary |
| **S2 harness** | Differential corpus + correctness gates, recreated independently of Nix | `tests/differential/**`, `tests/upstream/**` | S1 | `cargo test --workspace` green; `tests/differential/run.sh` runs fixture corpus vs pinned refs and is deterministic over 3 repeats |
| **S3 refresh** | Meson 1.12 / Ninja 1.13 deltas (`generator.process(depends:)`, build/subproject and cross semantics); upstream CLI shim (P-5) | `crates/meson/**`, `crates/ninja/**` | S2 | `tests/differential/run.sh` green against Meson 1.12.1 / Ninja 1.13.2 fixtures; upstream arg order accepted |
| **S4 harden frontend** | Compiler discovery, quoting (no string-joining for argv), path semantics, cross-compilation correctness | `crates/meson/**` | S3 | `cargo test -p meson` green; quoting/toolchain fixtures pass |
| **S5 harden executor** | Dirtiness, restat, dyndep, rspfile, subninja scopes, signals, jobserver, pools, missing-dep and interrupted-build correctness | `crates/ninja/**` | S3 | `cargo test -p ninja` green; executor fixtures pass |
| **S6 unify** | Shared typed graph + direct Meson-to-engine lowering; keep `build.ninja` generation | `crates/graph/**`, `crates/engine/**`, `crates/state/**` | S4, S5 | Equivalence test: direct-DAG build and `build.ninja`-reparsed build produce identical outputs and behavior on the corpus |
| **S7 optimize** | String interning, arena allocation, queue-driven scheduling, efficient metadata scans, persistent snapshot with conservative invalidation | `crates/graph/**`, `crates/engine/**`, `crates/state/**`, `tests/benchmarks/**` | S6 | `tests/benchmarks/run.sh` reports the corpus (below) with CPU/wall/RSS/syscalls on separate dirs; correctness gates stay green |
| **S8 qualify** | macOS arm64 + static-musl runs; `~/d/play` Meson/Ninja subset (OD-4) | `tests/upstream/**`, `.work/tmp/play-fixture/**` | S7 | `cargo build --release --target x86_64-unknown-linux-musl` succeeds; gluon `meson`+`ninja` build libplacebo 7.387.0 (play's pin, with its Metal patch) and the player crates link against it |
| **Close** | Delete dead code; `ultrareview`; fix P1/P2; final report | all owned files | S8 | Full suite green; report names accepted slices, parked items, and measured deltas |

Parallel pair: S4 and S5 may run concurrently in worktrees
`.work/wt/meson` and `.work/wt/ninja` (disjoint ownership). Everything else
serial. S4 and S5 both have prerequisite S3 only; neither may revise the
other's files.

## Acceptance

Each deliverable names its command and its distinct success/failure
conditions. No deliverable waits on the owner.

- **Both tools build and run** (`cargo build --release`): success = exit 0
  and `bins/meson`, `bins/ninja`, `bins/build` exist. Failure = any bin
  missing or `--version` non-zero.
- **Differential parity** (`tests/differential/run.sh`): success = every
  fixture's outputs, generated `build.ninja` (modulo documented
  normalization), and exit codes match the pinned reference. Covers:
  depfiles, dyndep (incl. eager merge + multi-producer), restat, response
  files, `subninja` scopes, interrupted builds, build-file regeneration,
  missing dependencies, pools, jobserver. Failure = any fixture mismatch
  (exit non-zero, no silent pass).
- **Upstream refresh** (`tests/differential/run.sh` with 1.12/1.13 pins):
  success = `generator.process(depends:)`, subproject/build semantics, and
  cross fixtures match Meson 1.12.1 / Ninja 1.13.2 behavior. Failure =
  fixture mismatch or CLI arg-order rejection (P-5).
- **Unification equivalence** (S6 equivalence test): success = direct-DAG
  and reparsed-`build.ninja` builds are byte/behavior-identical on the
  corpus. Failure = any divergence; the slice does not land.
- **Benchmark corpus** (`tests/benchmarks/run.sh`, S7): cold Meson setup;
  reconfigure; Ninja no-op at 1K/10K/100K edges; one-source and one-header
  incremental rebuild. Each reports CPU time, wall time, peak RSS, syscall
  count; separate equivalent build dirs per runner; caches/daemons off;
  median over repeats. Success = report produced and correctness gates
  green. Performance itself is logged, not gating, until Close.
- **Qualification** (S8): success = musl artifact builds; libplacebo +
  player crates build with gluon frontends on macOS arm64. Failure = any
  correctness divergence vs the `meson`+`muon`/`ninja` baseline in play's
  `scripts/build-deps.sh`.
- **Close**: success = full suite green, P1/P2 findings fixed, report
  present. Failure = open P1/P2 or any red.

## Needs environment

- **Muon 0.7.1+ / 0.8.0 canonical check.** Owner command:
  `git clone https://git.sr.ht/~lattis/muon /tmp/muon-check && muon version`
  (executor's host cannot reach SourceHut over https here). When run, update
  P-3 pins and S3 fixtures; until then the campaign pins Muon 0.7.0.
- **Windows compatibility.** Intentionally deferred; no executor command.
  Recorded so it is never mistaken for a passed gate.

## Execution boundary and non-goals

Authorized: edits inside the Sequence owned-file sets; local `cargo`,
`meson`, `ninja`, `muon` runs; fixture builds under `.work/` in this repo
and `~/d/play/.work/` (play's own scratch, read-only otherwise). No
formatters, no pushes, no network beyond pinned fetches, no changes to
`~/d/play` sources.
Forbidden: pushing; touching `~/d/play` tracked sources; benchmarks over
network/cached trees; portable artifacts with `-C target-cpu=native`;
GPL code in the tree (OD-2).
Contract changes (owner-visible): upstream-compatible CLI arg order (P-5);
`build.ninja` retained as the interchange until S6; engine/meson
separation as the stable internal boundary (P-2).
Non-goals: adopting n2, Orphis/ninja-rs, rninja (rejected: its checked-in
numbers show ~4.6× slower no-op and unstable hashing), or Siso (reference
only); a CMake frontend; network features; Windows as a design driver.

## Reference (verified at preflight, 2026-11-09)

- Oxidized Meson: `overby-me/oxidized-meson` @ `85dc226`, MIT, lexer/parser/
  bytecode VM → build graph → `build.ninja`; upstream 284/284 is Nix-only.
- Oxidized Ninja: `overby-me/oxidized-ninja` @ `bd297fe`, Apache-2.0,
  scheduler + dyndep/depfile/log/deps-log + jobserver; upstream 18/18 +
  4/4 + 7/7 is Nix-only. FNV-1a command hash (stable).
- `~/d/play` (proof target): Meson/Ninja used for libplacebo 7.387.0
  (`scripts/build-deps.sh`: muon preferred, meson fallback, samu/ninja
  backend, Metal patch); CMake used for Vulkan-Headers/Loader, shaderc,
  SPIRV-Cross — out of gluon scope per OD-4.
- Prior-art verdicts from the draft (kept, verified only where noted):
  n2 (architecture), Android fork (parser optimization), Orphis/ninja-rs
  (correctness reference). Full research history is in git history.
