# tokio-stdin

Read from stdin as a Tokio stream by spawning a separate thread.

```rust
extern crate futures;
extern crate tokio_stdin;

fn main() {
    use futures::Stream;

    tokio_stdin::spawn_stdin_stream_unbounded().wait();
}
```

As far as I know, this is currently the recommended way to do this. On Dec 29, 2016,
alexcrichton [commented](https://github.com/alexcrichton/tokio-process/issues/7):

> In general for small CLI tools and such what you probably want to do is to use channels to
> communicate to foreign threads. You can have a thread per stdin/stdout/stderr with a
> `futures::sync::mpsc` that the main thread communicates with.

This crate locks stdin while it's running, so trying to read from stdin in another part of your
code will probably cause a deadlock.

See the `count_keys` example for a simple use of this.

License: MIT/Apache-2.0
