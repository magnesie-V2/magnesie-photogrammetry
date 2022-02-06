use serde::{Deserialize, Serialize};

/// Response to a job status request
#[derive(Serialize, Deserialize, Debug)]
pub struct JobStatusResponse {
    pub status: String,
}

/// Response to a job info request
#[derive(Serialize, Deserialize, Debug)]
pub struct JobInfoResponse {
    pub status: String,
    pub logs: String,
}

/// Response to a job info request
#[derive(Serialize, Deserialize, Debug)]
pub struct JobPowerResponse {
    pub status: String,
    pub power: String,
}

/// Response to a job creation request
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobResponse {
    pub id: String,
}
