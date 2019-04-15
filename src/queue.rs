// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sequence::Sequence;

pub struct Queue {
    q: Vec<Sequence>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { q: vec![] }
    }

    pub fn push(&mut self, seq: Sequence) -> bool {
        if !self.q.contains(&seq) {
            self.q.push(seq);
            return true;
        }
        false
    }

    // pub fn pop(&mut self) -> Option<Sequence> {
    //     if self.q.len() > 0 {
    //         Some(self.q.remove(0))
    //     } else {
    //         None
    //     }
    // }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn dump(&mut self) {
        self.q = vec![];
    }

    pub fn seq_by_user(&mut self, user: String) -> Option<Sequence> {
        for idx in 0..self.q.len() {
            if user == self.q[idx].get_user() {
                return Some(self.q.remove(idx));
            }
        }
        return None;
    }
}
