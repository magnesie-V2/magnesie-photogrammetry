#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::response::content;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/job")]
fn create_job() -> status::Accepted<content::Json<&'static str>>{
    status::Accepted(Some(content::Json("{ \"id\": 0 }")))
}

#[get("/job/<id>")]
fn info_job(id:usize) -> content::Json<&'static str>{
    content::Json("{ \"status\": pending }")
}

fn main() {
    rocket::ignite().mount("/", routes![index, create_job, info_job]).launch();
}