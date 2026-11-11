# TODO

Prefix: GLU
Next: 19

### GLU-14 · P1 · setup ignores built-in layout and build options
- **Area:** meson
- **Where:** `crates/meson/src/cli.rs:197`
- **Problem:** `--prefix`, `--libdir`, `--buildtype`, `--default-library` and the other `--name=value` built-ins are parsed and discarded, so `get_option('buildtype')` reports `debug` and install directories fall back to defaults. Play's `scripts/build-deps.sh` passes all four.
- **Recommendation:** Route `--name=value` and `--name value` built-ins into the same option table as `-Dname=value`, and reject unknown ones.
- **Fix with:** reference-parity
- **Effort:** S

### GLU-15 · P1 · compile commands omit project, visibility, shared and optimization flags
- **Area:** meson
- **Where:** `crates/meson/src/builtins/functions.rs:4023`, `crates/meson/src/backend.rs:192`
- **Problem:** `add_project_arguments`, `add_project_link_arguments` and `add_global_arguments` are no-ops, and `gnu_symbol_visibility`, `c_shared_args` and the buildtype's optimization level never reach `ARGS`. On libplacebo 7.387.0 with Play's Metal patch, Meson 1.12.1 compiles with `-O3 -fvisibility=hidden -D_GNU_SOURCE -DPL_EXPORT -Wundef ...`; gluon passes only `-std=c11 -DPL_HAVE_PTHREAD` and includes.
- **Recommendation:** Store project and global arguments per language and machine, apply buildtype, visibility and per-kind args in the backend, and add a differential fixture that compares the normalized command set.
- **Fix with:** reference-parity
- **Effort:** M

### GLU-16 · P1 · install is not implemented
- **Area:** meson
- **Where:** `crates/meson/src/backend.rs:663`, `crates/meson/src/cli.rs:1000`, `crates/meson/src/backend.rs:894`
- **Problem:** `build.ninja` has no `install` target, so `ninja -C build install` fails with `unknown target 'install'`. `meson install` prints `Installing into /usr/local` regardless of the configured prefix and then runs that missing target. `intro-installed.json` is always `{}`. OD-4's Play gate never exercised Play's `ninja install` path.
- **Recommendation:** Write an install plan at setup from targets, headers, data, configure files, custom targets and symlinks under the configured directories; emit `install: phony meson-internal__install` depending on `all` and running `meson install --no-rebuild`; make `meson install` execute that plan with `DESTDIR`; fill `intro-installed.json` from it.
- **Fix with:** reference-parity
- **Effort:** L

### GLU-17 · P2 · soversion and version do not shape shared library names
- **Area:** meson
- **Where:** `crates/meson/src/builtins/functions.rs:871`, `crates/meson/src/backend.rs:387`
- **Problem:** `library('placebo', soversion: '387')` links `libplacebo.dylib` with `-install_name,libplacebo.dylib`. Meson 1.12.1 links `libplacebo.387.dylib` with `-install_name,@rpath/libplacebo.387.dylib,-compatibility_version,387,-current_version,387` and installs the `libplacebo.dylib` alias; Linux uses `libX.so.N` with a matching SONAME.
- **Recommendation:** Compute output name, SONAME or install name, Darwin versions and install aliases from `version`, `soversion` and `darwin_versions` as Meson does.
- **Fix with:** reference-parity
- **Effort:** M

### GLU-18 · P2 · pkgconfig.generate writes incomplete files and hides write errors
- **Area:** meson
- **Where:** `crates/meson/src/modules/pkgconfig.rs:12`
- **Problem:** `libraries:` is ignored, so libplacebo's `.pc` has no `Libs`; `prefix` is relative to `${pcfiledir}` instead of the configured prefix; `Requires.private` and `Libs.private` are not derived from the library's dependencies; write failures are discarded with `let _ =`.
- **Recommendation:** Generate from the library objects and their dependencies the way Meson does, use the configured prefix and directories, and return write errors.
- **Fix with:** reference-parity
- **Effort:** M
