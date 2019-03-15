// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::queue::Queue;
use crate::sequence::Sequence;
use crate::utils::{check_and_create_dir, is_empty};
use crate::LOG;
use failure::Error;
use slog::{info, warn};
use std::fs::{create_dir, read_dir, rename};
use std::path::PathBuf;

pub struct Scheduler {
    inbox: PathBuf,
    queued: PathBuf,
    outbox: PathBuf,
    q: Queue,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            inbox: PathBuf::from("inbox"),
            queued: PathBuf::from("queued"),
            outbox: PathBuf::from("outbox"),
            q: Queue::new(),
        }
    }

    pub fn inbox(mut self, dir: &str) -> Self {
        self.inbox = PathBuf::from(dir);
        self
    }

    pub fn queued(mut self, dir: &str) -> Self {
        self.queued = PathBuf::from(dir);
        self
    }

    pub fn outbox(mut self, dir: &str) -> Self {
        self.outbox = PathBuf::from(dir);
        self
    }

    pub fn run(mut self) -> Result<!, Error> {
        info!(LOG, "Your choices for the directories were: ");
        info!(LOG, ""; "inbox" => self.inbox.to_str().unwrap(),
                       "queued" => self.queued.to_str().unwrap(),
                       "outbox" => self.outbox.to_str().unwrap());

        self.make_dirs()?;
        self.collect_leftovers()?;

        loop {
            self.check_dirs()?;
            let new_in = self.hit_refresh()?;
            let new_sched = self.schedule()?;

            if new_in || new_sched {
                let len = self.q.len();
                if len == 0 {
                    info!(LOG, "Queue empty! Hooray!");
                } else {
                    info!(LOG, "Current length of queue: {}", len);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    fn make_dirs(&mut self) -> Result<(), Error> {
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

    fn collect_leftovers(&mut self) -> Result<(), Error> {
        for leftover in read_dir(&self.queued)? {
            let leftover = leftover?;
            let path = leftover.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                info!(LOG, "Found leftover: {}", filename);
                self.q.push(Sequence::new(filename.to_string()));
            }
        }
        Ok(())
    }

    fn check_dirs(&mut self) -> Result<(), Error> {
        if check_and_create_dir(&self.inbox)? {
            warn!(
                LOG,
                "I had to create the inbox folder again! Whatever wasn't queued already is lost :("
            );
        }
        if check_and_create_dir(&self.queued)? {
            warn!(LOG, "I had to create the queued folder again! This means that the queue is also lost :(");
            // flush queue because if queued does not exist, queue must be empty!
            self.q.dump();
        }
        if check_and_create_dir(&self.outbox)? {
            warn!(
                LOG,
                "I had to create the outbox folder again! I don't know what happend to your job :("
            );
        }
        Ok(())
    }

    fn hit_refresh(&mut self) -> Result<bool, Error> {
        let mut changes = false;
        // get new files
        for file in read_dir(&self.inbox)? {
            let file = file?;
            let path = file.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                info!(LOG, "New sequence: {}", filename);
                rename(&path, &self.queued.join(&filename))?;
                if !self.q.push(Sequence::new(filename.to_string())) {
                    warn!(LOG, "File {} already in queue!", filename);
                }
                changes = true;
            }
        }
        Ok(changes)
    }

    fn schedule(&mut self) -> Result<bool, Error> {
        let mut changes = false;
        if is_empty(&self.outbox)? {
            if let Some(seq) = self.q.pop() {
                rename(
                    &self.queued.join(&seq.get_name()),
                    &self.outbox.join("external.txt"),
                )?;
                info!(LOG, "Scheduling: {}", seq.get_name());
                changes = true;
            }
        }
        Ok(changes)
    }
}
