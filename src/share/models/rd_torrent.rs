use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Torrent {
    pub info_hash: String,
    pub status: String,
    pub files: Vec<String>,
}
