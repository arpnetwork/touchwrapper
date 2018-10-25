// Copyright 2018 ARP Network
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem::{size_of_val, transmute};
use std::os::unix::io::IntoRawFd;
use std::process;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

enum Cmd {
    Req((u8, u32)),
    Resp((u8, u32)),
}

#[repr(C)]
#[derive(Default)]
struct Event {
    time: u64,
    type_: u16,
    code: u16,
    value: u32,
}

fn main() {
    if env::args().len() != 2 {
        return;
    }

    let (tx, rx) = channel();
    let put_tx = tx.clone();

    thread::spawn(move || {
        let file = env::args().nth(1).unwrap();
        let fd = File::open(file).unwrap().into_raw_fd();

        let event = Event::default();
        let buf = unsafe { transmute(&event) };
        let count = size_of_val(&event);

        let send = |type_, value| tx.send(Cmd::Resp((type_, value))).unwrap();

        loop {
            if unsafe { libc::read(fd, buf, count) } == count as isize {
                if event.type_ == 3 {
                    if event.code == 53 {
                        send(0, event.value)
                    } else if event.code == 54 {
                        send(1, event.value)
                    }
                }
            } else {
                process::exit(-1);
            }
        }
    });

    thread::spawn(move || {
        let send = |type_, value: &str| {
            if let Ok(value) = value.parse() {
                put_tx.send(Cmd::Req((type_, value))).unwrap();
            }
        };

        let mut buf = String::new();
        while let Ok(size) = io::stdin().read_line(&mut buf) {
            if size == 0 {
                process::exit(-1);
            }

            {
                let cmd = buf.trim();
                let items: Vec<_> = cmd.split_whitespace().collect();
                if cmd.starts_with("w") && items.len() == 2 {
                    if let Ok(ms) = items[1].parse() {
                        thread::sleep(Duration::from_millis(ms));
                    }
                } else {
                    if items.len() == 7 {
                        send(0, items[2]);
                        send(1, items[3]);
                    }

                    println!("{}", cmd);
                    io::stdout().flush().unwrap();
                }
            }

            buf.clear();
        }
    });

    // Do some useful work for awhile

    let mut values: HashMap<_, Instant> = HashMap::new();

    let make_key = |(t, v)| ((t as u64) << 32) + v as u64;

    // Let's see what that answer was
    for item in rx {
        values.retain(|_, &mut value| {
            let elapsed = value.elapsed();
            elapsed.as_secs() == 0 && elapsed.subsec_millis() < 500
        });

        match item {
            Cmd::Req(key) => {
                let key = make_key(key);
                values.insert(key, Instant::now());
            }
            Cmd::Resp(key) => {
                let key = make_key(key);
                if !values.contains_key(&key) {
                    process::exit(-1);
                }
            }
        }
    }
}
