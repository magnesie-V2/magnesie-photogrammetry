use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateJobRequest {
    pub photos: Vec<String>,
    pub callback: String
}