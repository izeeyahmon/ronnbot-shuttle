use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) channel_id: u64,
    pub(crate) emotes: Vec<String>,
    pub(crate) role_ids: Vec<u64>,
}
