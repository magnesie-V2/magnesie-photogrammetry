use std::process::{Child, Command};
use uuid::Uuid;

use crate::env::{get_var, PHOTOGRAMMETRY_SCRIPT};
use crate::job::params::request::CreateJobRequest;
use std::net::IpAddr;

#[derive(strum_macros::ToString, Debug)]
pub enum Status {
    InProgress,
    Finished,
    Error,
}

pub struct Job {
    uuid: Uuid,
    child: Child,
    request: CreateJobRequest,
}

impl Job {
    pub fn new(request: CreateJobRequest, ip: IpAddr) -> Self {
        let uuid = Uuid::new_v4();

        let child = Command::new(get_var(PHOTOGRAMMETRY_SCRIPT))
            .arg(&uuid.to_string())
            .arg(format!(
                "{}:{}{}",
                ip.to_string(),
                7878,
                &request.callback.replace("<id>", &*uuid.to_string())
            ))
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

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn child(&self) -> &Child {
        &self.child
    }

    pub fn request(&self) -> &CreateJobRequest {
        &self.request
    }
}
