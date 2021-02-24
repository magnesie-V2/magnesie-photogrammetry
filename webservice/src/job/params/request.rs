use serde::{Deserialize, Serialize};

/// Job creation request
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobRequest {
    pub photos: Vec<String>,
    pub callback: String,
}
