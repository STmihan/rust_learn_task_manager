use serde_derive::{Deserialize, Serialize};
use crate::task::Task;

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub tasks: Vec<Task>,
}
