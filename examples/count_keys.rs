#![deny(warnings)]
//! If you're on a Unix system, try something like:
//!
//! `yes | cargo run --example count_keys`
extern crate futures;
extern crate tokio_stdin;
extern crate tokio_timer;

use futures::stream::{once, Stream};
use std::time::Duration;
use tokio_stdin::spawn_stdin_stream_unbounded;
use tokio_timer::{Timer, TimerError};

#[derive(Debug)]
enum Error {
    Timer(TimerError),
    Stdin(()),
}

enum Event {
    Byte,
    Second,
    Done,
}

fn main() {
    let seconds_stream = Timer::default()
        .interval(Duration::from_secs(1))
        .map(|()| Event::Second)
        .map_err(Error::Timer);

    let stdin_stream = spawn_stdin_stream_unbounded()
        .map(|_| Event::Byte)
        .map_err(Error::Stdin)
        .chain(once(Ok(Event::Done)));

    let rate = stdin_stream.select(seconds_stream);

    let mut n_bytes = 0;
    let mut n_seconds = 0;

    for event in rate.wait() {
        match event {
            Ok(Event::Byte) => n_bytes += 1,
            Ok(Event::Second) => {
                n_seconds += 1;
                println!("{} bytes in {} seconds", n_bytes, n_seconds);
            }
            Ok(Event::Done) => {
                println!("{} bytes in {} seconds", n_bytes, n_seconds);
                return;
            }
            Err(e) => eprintln!("error {:?}", e),
        }
    }
}
