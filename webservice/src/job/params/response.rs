use serde::{Deserialize, Serialize};

/// Response to a job info request
#[derive(Serialize, Deserialize, Debug)]
pub struct JobInfoResponse {
    pub status: String,
}

/// Response to a job creation request
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobResponse {
    pub id: String,
}
