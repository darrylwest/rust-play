# Pipe Viewer

This code originated from the training video [Hands On System Programming with Rust](https://www.udemy.com/course/hands-on-systems-programming-with-rust/) and has been enhanced to add:

* configuration struct as parsed from cli
* macro based cli parsing
* added logging
* unit and integrtion tests

The application reads from a file or stdin and writes to stdout or a specified file while gathering stats on the piped stream.  Read, Write and stats are gathered in three threads that communicate via channels.

## Testers

* tests/quick-test.sh

## Crates Used

* [clap](https://crates.io/crates/anyhow) - for cli in macro/derive mode
* [anyhow](https://crates.io/crates/anyhow) - error handling
* [crossbeam](https://crates.io/crates/crossbeam) - channels
* [crossterm](https://crates.io/crates/crossterm) - terminal output of stats
* [log](https://crates.io/crates/log) and [env_logger](https://crates.io/crates/env_logger) - logging

## To Do

* logging with cli switch to set level (default error)
* improved error handling and error logging
* unit and integration tests

###### darryl.west | 2022-09-07
