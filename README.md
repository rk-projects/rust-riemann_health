# Rust implementation of riemann_health.

This is an implementation of riemann_health in rust. 
a few features have been implemented and others would follow. 
This project uses riemann_client rust plugin (https://github.com/borntyping/rust-riemann_client) version "0.7.0" which contains addition of attributes to event.

### Build and Run 

To print help menu
```Rust 
cargo build && cargo run -- -h
```

Execute by passing argument with -c as <hostname>:<port> .
Let's say riemann server is running on localhost with port 5555.
```Rust
cargo build && cargo run -- -c localhost:5555 
```

The events are currently sent with a delay of 500ms between each event. As of version 0.1.0, this is not configurable. Later versions are expected to make this configurable.

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
