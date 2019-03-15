// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! funnel
//!
//! A likely error-prone bottleneck.

use failure::Error;
use lazy_static::lazy_static;
use slog::{info, o, Drain};
use std::fs::{create_dir, read_dir, rename};
use std::path::Path;

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

struct Sequence {
    name: String,
}

impl Sequence {
    pub fn new(name: String) -> Self {
        Sequence { name }
    }
}

struct Queue {
    q: Vec<Sequence>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { q: vec![] }
    }

    pub fn push(&mut self, seq: Sequence) {
        self.q.push(seq);
    }

    pub fn pop(&mut self) -> Option<Sequence> {
        if self.q.len() > 0 {
            Some(self.q.remove(0))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn dump(&mut self) {
        self.q = vec![];
    }
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
    // let decorator = slog_term::TermDecorator::new().build();
    // let drain = slog_term::FullFormat::new(decorator).build().fuse();
    // let drain = slog_async::Async::new(drain).build().fuse();
    // let log: () = slog::Logger::root(drain, o!());

    info!(LOG, "------------------------------------------------");
    info!(LOG, "| Welcome!                                     |");
    info!(LOG, "| I'm funnel, a likely error-prone bottleneck! |");
    info!(LOG, "------------------------------------------------");
    info!(LOG, "Your choices for the directories were: ");
    info!(LOG, ""; "inbox" => INBOX, "queued" => QUEUED, "outbox" => OUTBOX);

    let inbox = Path::new(INBOX);
    let outbox = Path::new(OUTBOX);
    let queued = Path::new(QUEUED);

    let mut q = Queue::new();

    for leftover in read_dir(queued)? {
        let leftover = leftover?;
        let path = leftover.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            info!(LOG, "Found leftover: {}", filename);
            q.push(Sequence::new(filename.to_string()));
        }
    }

    loop {
        if check_and_create_dir(inbox)? {
            info!(
                LOG,
                "I had to create the inbox folder again! Whatever wasn't queued already is lost :("
            );
        }
        if check_and_create_dir(queued)? {
            info!(LOG, "I had to create the queued folder again! This means that the queue is also lost :(");
            q.dump();
        }
        if check_and_create_dir(outbox)? {
            // flush queue because if queued does not exist, queue must be empty!
            info!(
                LOG,
                "I had to create the outbox folder again! I don't know what happend to your job :("
            );
        }

        let mut changes = false;
        // get new files
        for file in read_dir(inbox)? {
            let file = file?;
            let path = file.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                info!(LOG, "New sequence: {}", filename);
                rename(&path, queued.join(&filename))?;
                q.push(Sequence::new(filename.to_string()));
                changes = true;
            }
        }

        if is_empty(outbox)? {
            if let Some(seq) = q.pop() {
                rename(queued.join(&seq.name), outbox.join("external.txt"))?;
                info!(LOG, "Scheduling: {}", seq.name);
                changes = true;
            }
        }

        if changes {
            let len = q.len();
            if len == 0 {
                info!(LOG, "Queue empty! Hooray!");
            } else {
                info!(LOG, "Current lenghth of queue: {}", q.len());
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn main() {
    if let Err(ref e) = run() {
        println!("{}: {}", e.as_fail(), e.backtrace());
        std::process::exit(1);
    }
}
