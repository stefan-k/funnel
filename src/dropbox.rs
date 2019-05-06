// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use shellfn::shell;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum DropboxStatus {
    UpToDate,
    Syncing,
}

pub fn dropbox_filestatus(dir: &str) -> DropboxStatus {
    let info = dropbox_filestatus_int(dir);
    //println!("|{:?}|", info);
    if info == " up to date\n" {
        DropboxStatus::UpToDate
    } else {
        DropboxStatus::Syncing
    }
}

#[shell]
fn dropbox_filestatus_int(dir: &str) -> String {
    r#"
    dropbox-cli filestatus $DIR | awk '{split($0,a,":"); print a[2]}'
    "#
}
