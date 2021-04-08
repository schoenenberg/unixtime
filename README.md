# unixtime 

[![Rust](https://github.com/schoenenberg/unixtime/actions/workflows/rust.yml/badge.svg)](https://github.com/schoenenberg/unixtime/actions/workflows/rust.yml)

A small utility to print the current unix-time on STDOUT.

## Motivation

Sometimes I need the current unix-time for interacting with APIs and other stuff. But I never remember the correct command for it (is's `date +%s`). I mostly end up googling the current unix-time..

As it's using `chrono` as date and time library this should also work on Windows and other systems not having the `date` command.

Late at night, I quickly built this tool. Maybe someone is having the same issue.

It's fast as the ususal `date` tool:

```
$ time unixtime
1617888769
unixtime  0.00s user 0.00s system 83% cpu 0.001 total
$ time date +%s
1617888769
date +%s  0.00s user 0.00s system 77% cpu 0.001 total
```

## Installation

Installation requires the Rust-Toolchain. Then install it by executing:
```bash
cargo install unixtime
```

Then you should be able to launch `unixtime`.
