//! Shared execution engine (S7). Executes a `graph::State` directly
//! without going through build.ninja text.

pub fn execute(state: &graph::State, opts: &ninja::cli::Options) -> Result<u8, String> {
    ninja::build::run(state, opts)
}
