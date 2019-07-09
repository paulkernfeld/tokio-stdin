//! *NOTE:* This library is no longer necessary. As of `tokio` 0.1.6, it is now possible to read
//! from stdin without the overhead of spawning a separate thread with the `tokio::io::stdin`
//! function.
//!
//! Read from stdin as a Tokio stream by spawning a separate thread.
//!
//! ```rust
//! extern crate futures;
//! extern crate tokio_stdin;
//!
//! fn main() {
//!     use futures::Stream;
//!
//!     tokio_stdin::spawn_stdin_stream_unbounded().wait();
//! }
//! ```
//!
//! As far as I know, this is currently the recommended way to do this. On Dec 29, 2016,
//! alexcrichton [commented](https://github.com/alexcrichton/tokio-process/issues/7):
//!
//! > In general for small CLI tools and such what you probably want to do is to use channels to
//! > communicate to foreign threads. You can have a thread per stdin/stdout/stderr with a
//! > `futures::sync::mpsc` that the main thread communicates with.
//!
//! This crate locks stdin while it's running, so trying to read from stdin in another part of your
//! code will probably cause a deadlock.
//!
//! See the `count_keys` example for a simple use of this.
#![deny(missing_docs)]
#![deny(warnings)]
// TODO `futures::stream::iter` is deprecated but will be restored as `futures::stream::iter_result`
#![allow(deprecated)]
extern crate futures;

use futures::stream::iter;
use futures::sync::mpsc::{channel, unbounded, Receiver, SendError, UnboundedReceiver};
use futures::{Future, Sink, Stream};
use std::io::{self, Read};
use std::thread;

#[derive(Debug)]
enum Error {
    Stdin(std::io::Error),
    Channel(SendError<u8>),
}

/// Spawn a new thread that reads from stdin and passes messages back using a bounded channel.
pub fn spawn_stdin_stream_bounded(buffer: usize) -> Receiver<u8> {
    let (channel_sink, channel_stream) = channel(buffer);
    let stdin_sink = channel_sink.sink_map_err(Error::Channel);

    thread::spawn(move || {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        iter(stdin_lock.bytes())
            .map_err(Error::Stdin)
            .forward(stdin_sink)
            .wait()
            .unwrap();
    });

    channel_stream
}

/// Spawn a new thread that reads from stdin and passes messages back using an unbounded channel.
pub fn spawn_stdin_stream_unbounded() -> UnboundedReceiver<u8> {
    let (channel_sink, channel_stream) = unbounded();
    let stdin_sink = channel_sink.sink_map_err(Error::Channel);

    thread::spawn(move || {
        let stdin = io::stdin();
        let stdin_lock = stdin.lock();
        iter(stdin_lock.bytes())
            .map_err(Error::Stdin)
            .forward(stdin_sink)
            .wait()
            .unwrap();
    });

    channel_stream
}
