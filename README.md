# libueventd-rs

This library provides a clean Rust interface for working with device nodes
and sysfs. Functionality to watch for devices appearing and disappearing is
provided via an abstraction over inotify, so users just have to consume a
stream of events.

## Building

- Build targets:
  `mmm vendor/google/desktop/libs/libueventd-rs` from the repo root

- Run the unit tests:
  `atest libueventd_test libmbim_packets_test libmbim_api_generator_test`
