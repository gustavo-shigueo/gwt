use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_branch: String,
    pub remote: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_branch: "develop".to_owned(),
            remote: "origin".to_owned(),
        }
    }
}
