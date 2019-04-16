// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::backend::{Job, JobStatus};

pub struct Queue {
    q: Vec<Job>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { q: vec![] }
    }

    pub fn push(&mut self, job: Job) -> bool {
        if !self.q.contains(&job) {
            self.q.push(job);
            return true;
        }
        false
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn dump(&mut self) {
        self.q = vec![];
    }

    pub fn seq_by_user(&mut self, user: String) -> Option<Job> {
        for idx in 0..self.q.len() {
            if self.q[idx].status() == JobStatus::Queued && user == self.q[idx].user().to_string() {
                return Some(self.q.remove(idx));
            }
        }
        return None;
    }
}
