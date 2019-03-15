// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::LOG;
use failure::Error;
use slog::info;
use std::fs::{create_dir, read_dir};
use std::path::Path;

pub fn is_empty(path: &Path) -> Result<bool, Error> {
    for _ in read_dir(path)? {
        return Ok(false);
    }
    Ok(true)
}

pub fn check_and_create_dir(dir: &Path) -> Result<bool, Error> {
    if !dir.is_dir() {
        info!(
            LOG,
            "Whoops, something went wrong, {:?} does not exist! Let me fix this for you!", dir
        );
        create_dir(dir)?;
        return Ok(true);
    }
    Ok(false)
}
