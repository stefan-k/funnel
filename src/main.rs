// Copyright 2019 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! funnel
//!
//! A likely error-prone bottleneck.

#![feature(never_type)]

mod queue;
mod scheduler;
mod sequence;

use crate::queue::Queue;
use crate::scheduler::Scheduler;
use crate::sequence::Sequence;
use failure::Error;
use lazy_static::lazy_static;
use slog::{info, o, Drain};
use std::fs::{create_dir, read_dir, rename};
use std::path::{Path, PathBuf};

const INBOX: &'static str = "inbox/";
const OUTBOX: &'static str = "outbox/";
const QUEUED: &'static str = "queued/";

lazy_static! {
    static ref LOG: slog::Logger = {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        slog::Logger::root(drain, o!())
    };
}

fn is_empty(path: &Path) -> Result<bool, Error> {
    for _ in read_dir(path)? {
        return Ok(false);
    }
    Ok(true)
}

fn check_and_create_dir(dir: &Path) -> Result<bool, Error> {
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

fn run() -> Result<(), Error> {
    info!(LOG, "------------------------------------------------");
    info!(LOG, "| Welcome!                                     |");
    info!(LOG, "| I'm funnel, a likely error-prone bottleneck! |");
    info!(LOG, "------------------------------------------------");

    Scheduler::new()
        .inbox(INBOX)
        .queued(QUEUED)
        .outbox(OUTBOX)
        .run()?;
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}: {}", e.as_fail(), e.backtrace());
        std::process::exit(1);
    }
}
