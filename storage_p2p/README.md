## Peer to peer distributed file storage - server side ##

### What is this repository for? ###

* Distributed file storage, based on  peer-to-peer architecture that distributes the files content across a network of registered peers which share the storage with the network.
* v1.0.0

![badge](https://img.shields.io/badge/lang-Rust-green)
![badge](https://img.shields.io/badge/storage-in_progess...-blue)

At-a-glance:
- 💻 Server - backend
- 💡 Rust language
- 🐳 Container-based

## Overview

Rust implementation of the server side of a distributed storage system.

### Install the database

Check the [install_cassandra.md](install_cassandra.md) file for how to install the database and the drivers.

### How to build it

At first, install [Cargo and Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html). 

And then build the library with:

```bash
$ cargo build --release --all-targets
```

### How to run it

First set the environment variables
```bash
export CASSANDRA_HOST=127.0.0.1
export ZENOH_HOST=0.0.0.0
export RUST_LOG=debug
```

then run the application

```bash
$ ./target/release/examples/main
```

### Tests

See the `tests/` directory for usage.

To run all the test use:

```bash
$ cargo test
```

Otherwise specify the test name such as:

```bash
$ cargo test --test unit-tests
$ cargo test --test integration-tests
```

### Examples of usage

See the `examples/` directory for usage.


## Execute with Docker

### Execute with Dockerfile
```bash
$ cd <path_of_main_file>
$ docker build -t server -f <dockerfile_path> .
```

To run with default params:
```bash
$ docker run server 
```

To run with custom params:
```bash
$ docker run -p 0.0.0.0:7447:7447 -e CASSANDRA_HOST=172.17.0.1 -e ZENOH_HOST=0.0.0.0 server
```

### Execute with docker-compose
```bash
$ docker compose up -d
```
