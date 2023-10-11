use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub assignee: String,
    pub estimate: u8,
    pub complete: bool,
}