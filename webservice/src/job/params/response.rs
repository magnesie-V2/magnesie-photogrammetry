use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobInfoResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobResponse {
    pub id: String,
}
