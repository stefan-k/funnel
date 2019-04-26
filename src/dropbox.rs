// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// use failure::Error;
use shellfn::shell;
use std::error::Error;

enum DropboxStatus {
    UpToDate,
    Syncing,
}

impl std::str::FromStr for DropboxStatus {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "up to date" {
            Ok(DropboxStatus::UpToDate)
        } else if s == "sync" {
            Ok(DropboxStatus::Syncing)
        } else {
            // panic!("Unknown response from dropbox-cli.")
            Err("Unkown response from dropbox-cli".into())
        }
    }
}

#[shell]
pub fn dropbox_filestatus(dir: &str) -> String {
    r#"
    dropbox-cli filestatus $DIR
    "#
}
