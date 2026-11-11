# TODO

Prefix: GLU
Next: 21

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

### GLU-19 · P3 · shared libraries have no symbol-extractor edge
- **Area:** meson
- **Where:** `crates/meson/src/backend.rs`, `tests/differential/fixtures/meson/shared-args/skip-edge-count`
- **Problem:** Meson 1.12.1 adds a `SHSYM` edge per shared library so relinks skip dependents whose exported symbols did not change; gluon emits none, so the `gnu-symbol-visibility` and `shared-args` fixtures skip the edge-count comparison.
- **Recommendation:** Emit the symbol-extractor edge and make dependents' links depend on the `.symbols` output, then delete the `skip-edge-count` markers.
- **Fix with:** reference-parity
- **Effort:** M

### GLU-20 · P3 · install accepts options it does not apply
- **Area:** meson
- **Where:** `crates/meson/src/cli.rs`, `crates/meson/src/install.rs`
- **Problem:** `meson install --tags`, `--skip-subprojects` and `--strip` (and the `strip` option) are parsed and silently ignored, and a target's `install_mode` and `configure_file`'s `install_mode` never reach the install plan, so those installs get default modes.
- **Recommendation:** Apply each as Meson 1.12.1's `minstall.py` does, or reject it with an error until it is implemented.
- **Fix with:** reference-parity
- **Effort:** S
