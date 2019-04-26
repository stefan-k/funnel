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
#![allow(dead_code)]

mod accounting;
mod backend;
mod dropbox;
mod queue;
mod scheduler;
mod utils;

use crate::scheduler::Scheduler;
use dropbox::*;
use failure::Error;
use lazy_static::lazy_static;
use slog::{info, o, Drain};

lazy_static! {
    static ref LOG: slog::Logger = {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        slog::Logger::root(drain, o!())
    };
}

fn run() -> Result<(), Error> {
    info!(LOG, "------------------------------------------------");
    info!(LOG, "| Welcome!                                     |");
    info!(LOG, "| I'm funnel, a likely error-prone bottleneck! |");
    info!(LOG, "------------------------------------------------");

    Scheduler::new().run()?;
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}: {}", e.as_fail(), e.backtrace());
        std::process::exit(1);
    }
}
