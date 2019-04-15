// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::cmp::Ordering;

struct User {
    name: String,
    scheduled: usize,
    queued: usize,
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            name,
            scheduled: 0,
            queued: 1,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn increment_scheduled(&mut self) {
        self.scheduled += 1;
        self.queued -= 1;
    }

    pub fn increment_queued(&mut self) {
        self.queued += 1;
    }

    pub fn num_queued(&self) -> usize {
        self.queued
    }
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name
    }
}

impl Eq for User {}

impl Ord for User {
    fn cmp(&self, other: &User) -> Ordering {
        self.scheduled.cmp(&other.scheduled)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &User) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Accounting {
    users: Vec<User>,
}

impl Accounting {
    pub fn new() -> Self {
        Accounting { users: vec![] }
    }

    pub fn scheduled(&mut self, user: String) {
        let mut found = false;
        for i in 0..self.users.len() {
            if self.users[i].get_name() == user {
                self.users[i].increment_scheduled();
                found = true;
            }
        }

        if !found {
            let mut user = User::new(user);
            user.increment_scheduled();
            self.users.push(user);
            self.users.sort();
        }
    }

    pub fn queued(&mut self, user: String) {
        let mut found = false;
        for i in 0..self.users.len() {
            if self.users[i].get_name() == user {
                self.users[i].increment_queued();
                found = true;
            }
        }

        if !found {
            self.users.push(User::new(user));
            self.users.sort();
        }
    }

    pub fn next_user(&self) -> Option<String> {
        for user in &self.users {
            if user.num_queued() > 0 {
                return Some(user.get_name().to_string());
            }
        }
        None
    }
}
