use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Tenet {
    pub name: String,
    pub associated_practices: HashSet<String>,
    pub limited_practices: HashSet<String>,
}

impl Tenet {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

impl std::fmt::Display for Tenet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
