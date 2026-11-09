//! Build orchestration. Log/deps state lives in the `state` crate (S7);
//! re-exported here so existing `super::log` etc. paths keep working.

pub use state::depfile;
pub use state::deps_log;
pub use state::log;
mod dyndep;
mod expand;
mod jobserver;
pub mod plan;
mod runner;

pub use expand::expand_in_edge as expand_in_edge_pub;
pub use runner::{RunReport, run, run_targets};
