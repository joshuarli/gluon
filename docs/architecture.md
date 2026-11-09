# Architecture

Workspace layout: [`Cargo.toml`](../Cargo.toml) enumerates the members.

- `crates/graph` — the shared typed graph (commands, inputs, outputs,
  dep types, env, cwd, pools, dyndep metadata) both front ends lower to.
- `crates/state` — build logs and dep state (single owner; `.ninja_log`,
  deps/log persistence).
- `crates/engine` — the shared executor; never depends on Meson semantics.
- `crates/meson` — Meson front end (lexer/parser/bytecode VM → graph →
  `build.ninja` plus introspection files in `meson-info/`).
- `crates/ninja` — Ninja front end (parser, scheduler, dyndep/depfile/log,
  jobserver) reusing engine and state.
- `bins/meson`, `bins/ninja`, `bins/build` — thin wrappers. `bins/build`
  dispatches `gluon-build [meson|ninja] [args...]`.

Interchange: `build.ninja` generation is retained for external
compatibility; the `build` binary lowers Meson input directly to the typed
graph, and equivalence of the direct-DAG and reparsed-`build.ninja` paths is
covered by tests (see [testing.md](testing.md)).

CLI compatibility (the stable contract):

- Upstream Meson argument order: `setup <builddir> <srcdir>` works on the
  `meson` binary.
- `build.ninja` remains the interchange format for external tools.
- The engine/meson split is the stable internal boundary.

Not in scope: a CMake front end, network features, Windows compatibility
(the executor is intentionally not a Windows design driver).
