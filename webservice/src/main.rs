#![feature(proc_macro_hygiene, decl_macro)]

mod params;
mod job;

#[macro_use] extern crate rocket;

use rocket::response::status;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use rocket::State;
use std::sync::RwLock;
use rocket_contrib::json::Json;
use uuid::Uuid;

use crate::params::job_create_response::CreateJobResponse;
use crate::job::job::{Job, Status};
use crate::params::job_create_request::CreateJobRequest;
use crate::params::job_info_response::JobInfoResponse;

struct ProcessState {
    process: RwLock<HashMap<String,Job>>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/job", format = "json", data = "<job_request>")]
fn create_job(state: State<ProcessState>, job_request: Json<CreateJobRequest>) -> status::Accepted<Json<CreateJobResponse>>{

    let uuid = Uuid::new_v4();

    let child = Command::new("sh")
        .arg("-c")
        .arg(format!("curl {}",job_request.callback))
        .stderr(Stdio::null())
        .spawn()
        .expect("command failed to start");

    let job = Job { uuid, child };

    let mut lock = state.process.write()
        .expect("locking process map to write");
    lock.insert(uuid.to_string(), job);

    let response = CreateJobResponse { id: uuid.to_string() };

    status::Accepted(Some(Json(response)))
}

#[get("/job/<id>")]
fn info_job(state: State<ProcessState>, id:String) -> Option<Json<JobInfoResponse>>{
    let mut lock = state.process.write()
        .expect("locking process map to write");

    let status = match lock.get_mut(&*id) {
        None => Status::Unknown,
        Some(job) => job.get_status()
    };

    if status.is_unknown() {
        return None;
    }

    Some(Json(JobInfoResponse {status:status.to_string()}))
}

fn main() {
    let state = ProcessState {
        process: RwLock::new(HashMap::new())
    };
    rocket::ignite()
        .mount("/", routes![index, create_job, info_job])
        .manage(state)
        .launch();
}