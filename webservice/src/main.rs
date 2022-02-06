#![feature(proc_macro_hygiene, decl_macro)]

mod env;
mod job;

#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use std::sync::RwLock;

use env::check_env;
use job::job::Job;
use job::params::request::CreateJobRequest;
use job::params::response::CreateJobResponse;
use job::params::response::JobInfoResponse;
use job::params::response::JobPowerResponse;
use job::params::response::JobStatusResponse;

/// Route used to manually test if the service is up and running
#[get("/")]
fn index() -> &'static str {
    "Photogrammetry service is up and running"
}

/// Route called by the orchestrator to create a job
#[post("/job", format = "json", data = "<job_request>")]
fn create_job(
    state: State<ProcessState>,
    job_request: Json<CreateJobRequest>,
) -> status::Accepted<Json<CreateJobResponse>> {
    let job = Job::new(job_request.into_inner());

    let response = CreateJobResponse {
        id: job.uuid().to_string(),
    };

    let mut lock = state.process.write().expect("locking process map to write");
    lock.insert(job.uuid().to_string(), job);

    status::Accepted(Some(Json(response)))
}

/// Route used to access to a job's status
#[get("/job/status/<id>")]
fn job_status(state: State<ProcessState>, id: String) -> Option<Json<JobStatusResponse>> {
    let mut lock = state.process.write().expect("locking process map to write");

    match lock.get_mut(&*id) {
        None => None,
        Some(job) => Some(Json(JobStatusResponse {
            status: job.status().to_string()
        })),
    }
}

/// Route used to access to a job's info (status + mvgmvs logs)
#[get("/job/info/<id>")]
fn job_info(state: State<ProcessState>, id: String) -> Option<Json<JobInfoResponse>> {
    let mut lock = state.process.write().expect("locking process map to write");

    match lock.get_mut(&*id) {
        None => None,
        Some(job) => Some(Json(JobInfoResponse {
            status: job.status().to_string(),
            logs: job.logs(),
        })),
    }
}

/// Route used to access to a job's power (status + perf logs)
#[get("/job/power/<id>")]
fn job_power(state: State<ProcessState>, id: String) -> Option<Json<JobPowerResponse>> {
    let mut lock = state.process.write().expect("locking process map to write");

    match lock.get_mut(&*id) {
        None => None,
        Some(job) => Some(Json(JobPowerResponse {
            status: job.status().to_string(),
            power: job.power(),
        })),
    }
}

/// map that links any job id to the associated job
struct ProcessState {
    process: RwLock<HashMap<String, Job>>,
}

fn main() {
    check_env();
    let state = ProcessState {
        process: RwLock::new(HashMap::new()),
    };
    rocket::ignite()
        .mount("/", routes![index, create_job, job_status, job_info, job_power])
        .mount("/res", StaticFiles::from(env::get_var("RES_DIR")))
        .manage(state)
        .launch();
}
