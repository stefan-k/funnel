// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Sequence {
    user: String,
    name: String,
}

impl Sequence {
    pub fn new(user: String, name: String) -> Self {
        Sequence { user, name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_user(&self) -> String {
        self.user.clone()
    }
}
