use std::process::{Child, Command};

use crate::env::{get_var, PHOTOGRAMMETRY_SCRIPT, GET_POWER_SCRIPT};
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
    pub id: i32,
    child: Child,
}

impl Job {
    /// Job creation from orchestrator request
    pub fn new(request: CreateJobRequest) -> Self {
        let id: i32 = request.submission_id;
        // Surrounding photogrammetry script with perf command for measuring power consumption
        let perflogfile = format!("/logs/job/{}_perf", &id.to_string());

        let child = Command::new("perf")
            .arg("stat")
            .arg("--event")
            .arg("energy-cores")
            .arg("-o")
            .arg(perflogfile)
            .arg("--interval-print")
            .arg("5000")
            .arg("--field-separator")
            .arg(",")
            .arg(get_var(PHOTOGRAMMETRY_SCRIPT))
            .arg(&id.to_string())
            .arg(&request.callback.replace("<id>", &id.to_string()))
            .args(&request.photos)
            .spawn()
            .expect("job failed to start")
            ;

        let job = Job {
            id,
            child,
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

    /// Child process (Bash) that is computing the job
    pub fn child(&self) -> &Child {
        &self.child
    }

    /// Get photogrammetry logs since beginning
    pub fn logs(&self) -> String {
        let logfile = format!("/logs/job/{}", self.id.to_string());
        let output = Command::new("/bin/cat")
            .arg(logfile)
            .output()
            .expect("failed to read process perf logs");

        return String::from_utf8(output.stdout).unwrap();
    }

    /// Get photogrammetry power consumption since beginning
    pub fn power(&self) -> String {
        let output = Command::new(get_var(GET_POWER_SCRIPT))
            .arg(self.id.to_string())
            .output()
            .expect("failed to read process perf logs")
            ;

        return String::from_utf8(output.stdout).unwrap();
    }

    pub fn stop(&mut self) -> Result<(), String> {
        return match self.status() {
            Status::InProgress => {
                println!("Stopping job {}", self.id.to_string());
                Command::new("/bin/sed")
                    .arg("-i")
                    .arg("s/1/0/g")
                    .arg(&format!("/logs/job/{}_gonogo", self.id.to_string()))
                    .spawn()
                    .expect("failed to stop process")
                ;
                Ok(())
            },
            Status::Finished => Err(format!("Job {} has already been completed !", self.id.to_string())),
            Status::Error => Err(format!("Job {} process is unknown !", self.id.to_string()))
        };
    }

    pub fn start(&mut self) -> Result<(), String> {
        return match self.status() {
            Status::InProgress => {
                println!("Stopping job {}", self.id.to_string());
                Command::new("/bin/sed")
                    .arg("-i")
                    .arg("s/0/1/g")
                    .arg(&format!("/logs/job/{}_gonogo", self.id.to_string()))
                    .spawn()
                    .expect("failed to stop process")
                ;
                Ok(())
            },
            Status::Finished => Err(format!("Job {} has already been completed !", self.id.to_string())),
            Status::Error => Err(format!("Job {} process is unknown !", self.id.to_string()))
        };
    }

    /// Get photogrammetry current
    pub fn step(&self) -> String {
        let stepfile = format!("/logs/job/{}_step", self.id.to_string());
        let output = Command::new("/bin/cat")
            .arg(stepfile)
            .output()
            .expect("failed to read process step file");

        return String::from_utf8(output.stdout).unwrap();
    }
}
