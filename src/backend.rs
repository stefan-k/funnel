// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use failure::Error;
use std::fs::{create_dir, read_dir, rename};
use std::path::PathBuf;

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
enum JobStatus {
    Inbox,
    Queued,
    Running,
    Finished,
}

#[derive(Eq, PartialEq, Clone, Default, Ord, PartialOrd)]
pub struct User {
    name: String,
}

impl User {
    pub fn new<S: Into<String>>(name: S) -> User {
        User { name: name.into() }
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }
}

pub struct Job {
    user: User,
    id: String,
    status: JobStatus,
}

impl Job {
    pub fn new<S: Into<String>>(user: User, id: S) -> Job {
        Job {
            user,
            id: id.into(),
            status: JobStatus::Inbox,
        }
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn from_inbox_to_queued(&mut self) -> &mut Self {
        assert!(self.status == JobStatus::Inbox);
        self.status = JobStatus::Queued;
        self
    }

    pub fn from_queued_to_running(&mut self) -> &mut Self {
        assert!(self.status == JobStatus::Queued);
        self.status = JobStatus::Running;
        self
    }

    pub fn from_running_to_finished(&mut self) -> &mut Self {
        assert!(self.status == JobStatus::Running);
        self.status = JobStatus::Finished;
        self
    }
}

pub trait Backend {
    fn initialize(&self) -> Result<(), Error>;
    fn check_inbox(&self) -> Result<Vec<Job>, Error>;
    fn check_queued(&self) -> Result<Vec<Job>, Error>;
    fn from_inbox_to_queued(&self, job: Job) -> Result<Job, Error>;
    fn from_queued_to_running(&self, job: Job) -> Result<Job, Error>;
    fn from_running_to_finished(&self, job: Job) -> Result<Job, Error>;
}

pub struct Filesystem {
    inbox: PathBuf,
    queued: PathBuf,
    outbox: PathBuf,
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            inbox: PathBuf::from("inbox"),
            queued: PathBuf::from("queued"),
            outbox: PathBuf::from("outbox"),
        }
    }
}

impl Backend for Filesystem {
    fn initialize(&self) -> Result<(), Error> {
        if !self.inbox.is_dir() {
            create_dir(&self.inbox)?;
        }
        if !self.queued.is_dir() {
            create_dir(&self.queued)?;
        }
        if !self.outbox.is_dir() {
            create_dir(&self.outbox)?;
        }
        Ok(())
    }

    fn check_inbox(&self) -> Result<Vec<Job>, Error> {
        let mut out = vec![];
        // get new files
        for name in read_dir(&self.inbox)? {
            let name = name?.path();
            // only care about directories
            if name.is_dir() {
                let user = name.file_name().unwrap().to_str().unwrap();
                for job_name in read_dir(&name)? {
                    let job_name = job_name?.path();
                    // only files
                    if job_name.is_file() {
                        let job_name = job_name.file_name().unwrap().to_str().unwrap();
                        out.push(Job::new(User::new(user), job_name));
                    }
                }
            }
        }
        Ok(out)
    }

    fn check_queued(&self) -> Result<Vec<Job>, Error> {
        let mut out = vec![];
        // get new files
        for name in read_dir(&self.queued)? {
            let name = name?.path();
            // only care about directories
            if name.is_dir() {
                let user = name.file_name().unwrap().to_str().unwrap();
                for job_name in read_dir(&name)? {
                    let job_name = job_name?.path();
                    // only files
                    if job_name.is_file() {
                        let job_name = job_name.file_name().unwrap().to_str().unwrap();
                        out.push(Job::new(User::new(user), job_name));
                    }
                }
            }
        }
        Ok(out)
    }

    fn from_inbox_to_queued(&self, mut job: Job) -> Result<Job, Error> {
        job.from_inbox_to_queued();
        let from = self.inbox.join(job.user().to_string());
        let to = self.queued.join(job.user().to_string());
        rename(&from, &to)?;
        Ok(job)
    }

    fn from_queued_to_running(&self, mut job: Job) -> Result<Job, Error> {
        job.from_queued_to_running();
        let from = self.queued.join(job.user().to_string());
        let to = self.outbox.join(job.user().to_string());
        rename(&from, &to)?;
        Ok(job)
    }

    fn from_running_to_finished(&self, mut job: Job) -> Result<Job, Error> {
        job.from_running_to_finished();
        // let from = self.outbox.join(job.user().to_string());
        // let to = self.outbox.join(job.user().to_string());
        // rename(&from, &to)?;
        Ok(job)
    }
}
