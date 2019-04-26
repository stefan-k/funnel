// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::accounting::Accounting;
use crate::backend::*;
use crate::queue::Queue;
use crate::LOG;
use failure::Error;
use slog::info;
// use slog::{info, warn};

pub struct Scheduler {
    backend: Filesystem,
    q: Queue,
    acc: Accounting,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            backend: Filesystem::new(),
            q: Queue::new(),
            acc: Accounting::new(),
        }
    }

    pub fn run(mut self) -> Result<!, Error> {
        // Initialize Backend
        self.backend.initialize()?;

        // Print whatever the backend has to say
        self.backend.initial_log();

        // Check for leftovers and schedule them
        let queued_count = self
            .backend
            // Check for leftovers
            .check_queued()?
            // Iterate over leftovers
            .into_iter()
            .map(|x| -> Result<(), Error> {
                // Tell everyone about the discovery
                info!(LOG, "Leftover {} by user {}.", x.id(), x.user().to_string());
                // Tell accounting about it
                self.acc.queued(x.user().to_string());
                // Add to the queue
                self.q.push(x);
                Ok(())
            })
            .count();

        info!(LOG, "Found {} leftovers.", queued_count);

        // Loop forever
        loop {
            // Always initialize backend again, in case something happend in the meantime (i.e. a
            // directory was deleted)
            self.backend.initialize()?;

            let new_in_count = self
                .backend
                // Check the inbox for new jobs
                .check_inbox()?
                .into_iter()
                .map(|x| -> Result<(), Error> {
                    // Tell everyone that a new job was found
                    info!(LOG, "New job {} by user {}.", x.id(), x.user().to_string());
                    // Acknowledge the job's existence and queue it
                    let job = self.backend.from_inbox_to_queued(x)?;
                    // Inform accounting about it
                    self.acc.queued(job.user().to_string());
                    // Push job onto the queue
                    self.q.push(job);
                    Ok(())
                })
                .count();

            if new_in_count > 0 {
                info!(LOG, "Queued {} new jobs.", new_in_count);
            }

            // Schedule jobs
            self.schedule()?;

            // Wait a bit
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    fn schedule(&mut self) -> Result<bool, Error> {
        // If there is already a job running, don't do anything
        if !self.backend.something_running()? {
            // There is no job running, find out which user is next. Accounting will return only
            // users who have jobs queued, so there is no need to take care of that here.
            if let Some(user) = self.acc.next_user() {
                // Get the next job of the user. This should not return None in this context.
                // Note that `seq_by_user` will remove the job from the queue!
                if let Some(job) = self.q.seq_by_user(user) {
                    // change the jobs state from queued to running and move the files accordingly.
                    let job = self.backend.from_queued_to_running(job)?;
                    // Tell the people!
                    info!(LOG, "Scheduling {} by {}", job.id(), job.user().to_string());
                    // Inform accounting about the new running job
                    self.acc.scheduled(job.user().to_string());
                    // Push the job with updated state to the queue again.
                    self.q.push(job);
                } else {
                    // Since `next_user` is only giving us a user with a non-empty queue, this
                    // should not happen. If it does happen, do investigate.
                    panic!("This should not happen! Investigate!");
                }
            }
        }
        Ok(true)
    }
}
