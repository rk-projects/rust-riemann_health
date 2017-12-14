# Rust implementation of riemann_health.

This is an implementation of riemann_health in rust. 

This project uses riemann_client rust plugin (https://github.com/borntyping/rust-riemann_client) version "0.7.0" which contains addition of attributes to event.

The events are currently sent with a delay of 500ms between each event. This is configurable with -d parameter.

### Build and Run 

To print help menu
```Rust 
cargo build && cargo run -- -h
```

and the output would be 
``` Bash
$ cargo build && cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/rust-riemann_health -h`
Following is tbe help menu: 

Usage: target/debug/rust-riemann_health [options]

Options:
    -c, --connection [connection]
                        connection string to riemann server in the format
                        <hostname>:<port> . defaults to locahost:5555
    -d, --delay [delay] milliseconds of delay between events. defaults to
                        500ms.
    -h, --help          print help menu
$
```

To execute without any arguments : ( defaults to localhost:5555 for server connection and 500ms for delay)
```Rust
cargo run 
```
or 
```Rust
cargo run -- -c -d 
```

Execute by passing argument with -c as hostname:port and -d as time in milliseconds
Let's say riemann server is running on localhost with port 5555 and you wish to send events with a delay of 1 second
Both parameters, connection and delay are optional. Print help to see defaults.
```Rust
cargo run -- -c localhost:5555 -d 1
```

### Metrics Sent

Currently, the following metrics are being sent.
* boottime
* disk
  * total space in bytes
  * free space in bytes 
* memory 
  * total memory
  * free memory 
  * cached memory
  * buffer memory
  * available memory
  * total swap memory
  * free swap memory
