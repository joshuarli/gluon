//! Explicit build graph shared by frontends (P-2). Commands keep shell
//! semantics (structured argv deferred to S8).

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct State {
    pub bindings: HashMap<String, String>,
    pub rules: HashMap<String, Rule>,
    pub edges: Vec<Edge>,
    pub producers: HashMap<String, usize>,
    pub defaults: Vec<String>,
    pub pools: HashMap<String, usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Rule {
    pub name: String,
    pub bindings: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub rule: String,
    pub outputs: Vec<String>,
    pub implicit_outputs: Vec<String>,
    pub inputs: Vec<String>,
    pub implicit_inputs: Vec<String>,
    pub order_only_inputs: Vec<String>,
    pub bindings: HashMap<String, String>,
    pub dyndep: Option<String>,
}

impl Edge {
    pub fn is_phony(&self) -> bool {
        self.rule == "phony"
    }

    pub fn command(&self) -> Option<&str> {
        self.bindings.get("command").map(String::as_str)
    }
}
