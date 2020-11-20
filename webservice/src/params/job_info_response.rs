use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JobInfoResponse {
    pub status: String
}