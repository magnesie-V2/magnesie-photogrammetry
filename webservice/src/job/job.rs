use std::process::{Child, Command};
use uuid::Uuid;

use crate::env::{get_var, PHOTOGRAMMETRY_SCRIPT};
use crate::job::params::request::CreateJobRequest;

/// Status of a job
#[derive(strum_macros::ToString, Debug)]
pub enum Status {
    InProgress,
    Finished,
    Error,
}

/// Job description
pub struct Job {
    uuid: Uuid,
    child: Child,
    request: CreateJobRequest,
}

impl Job {
    /// Job creation from orchestrator request
    pub fn new(request: CreateJobRequest) -> Self {
        let uuid = Uuid::new_v4();

        let child = Command::new(get_var(PHOTOGRAMMETRY_SCRIPT))
            .arg(&uuid.to_string())
            .arg(&request.callback.replace("<id>", &*uuid.to_string()))
            .args(&request.photos)
            .spawn()
            .expect("job failed to start");

        let job = Job {
            uuid,
            child,
            request,
        };

        job
    }

    /// Job status computation
    pub fn status(&mut self) -> Status {
        let opt = self.child.try_wait().expect("error while fetching status");
        let status = match opt {
            None => Status::InProgress,
            Some(status) => {
                if status.success() {
                    Status::Finished
                } else {
                    Status::Error
                }
            }
        };
        return status;
    }

    /// Uuid of the job
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Child process (Bash) that is computing the job
    pub fn child(&self) -> &Child {
        &self.child
    }

    /// Request that got this job created
    pub fn request(&self) -> &CreateJobRequest {
        &self.request
    }
}
