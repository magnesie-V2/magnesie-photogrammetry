use uuid::Uuid;
use std::process::Child;



#[derive(strum_macros::ToString, Debug)]
pub enum Status {
    InProgress,
    Finished,
    Error,
    Unknown
}

impl Status {
    pub fn is_unknown(&self) -> bool {
        match *self {
            Status::Unknown => true,
            _ => false,
        }
    }
}


pub struct Job {
    pub uuid: Uuid,
    pub child: Child
}

impl Job {
    pub fn get_status(&mut self) -> Status {
        let opt = self.child.try_wait().expect("error while fetching status");
        let status = match opt {
            None => Status::InProgress,
            Some(status) => if status.success() { Status::Finished} else {Status::Error}
        };
        return status;
    }
}