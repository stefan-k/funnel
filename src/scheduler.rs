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
        self.backend.initial_log();
        self.backend.initialize()?;
        let queued_count = self
            .backend
            .check_queued()?
            .into_iter()
            .map(|x| -> Result<(), Error> {
                info!(LOG, "Leftover {} by user {}.", x.id(), x.user().to_string());
                self.acc.queued(x.user().to_string());
                self.q.push(x);
                Ok(())
            })
            .count();

        info!(LOG, "Found {} leftovers.", queued_count);

        loop {
            self.backend.initialize()?;
            let _new_in_count = self
                .backend
                .check_inbox()?
                .into_iter()
                .map(|x| -> Result<(), Error> {
                    info!(LOG, "New job {} by user {}.", x.id(), x.user().to_string());
                    let job = self.backend.from_inbox_to_queued(x)?;
                    self.acc.queued(job.user().to_string());
                    self.q.push(job);
                    Ok(())
                })
                .count();

            self.schedule()?;

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    fn schedule(&mut self) -> Result<bool, Error> {
        if !self.backend.something_running()? {
            if let Some(user) = self.acc.next_user() {
                if let Some(job) = self.q.seq_by_user(user) {
                    let job = self.backend.from_queued_to_running(job)?;
                    info!(LOG, "Scheduling {} by {}", job.id(), job.user().to_string());
                    self.acc.scheduled(job.user().to_string());
                    self.q.push(job);
                }
            }
        }
        Ok(true)
    }
}
