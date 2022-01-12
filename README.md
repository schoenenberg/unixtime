# unixtime 

[![Rust](https://github.com/schoenenberg/unixtime/actions/workflows/rust.yml/badge.svg)](https://github.com/schoenenberg/unixtime/actions/workflows/rust.yml)

A small utility for working with UNIX time.

## Motivation
Sometimes I need the current unix-time for interacting with APIs and other stuff. But I never remember the correct command for it (it's `date +%s`). I mostly end up googling the current unix-time. Late at night, I quickly built the original version of this tool. Maybe someone is having the same issue.

## Features

- Printing the current UNIX timestamp in seconds, milliseconds or nanoseconds
- Reverse conversion of UNIX timestamp to human readable format (RFC 2822, RFC 3339)

As it's using `chrono` as date and time library this should also work on Windows and other systems not having the `date` command.

## Installation

### Download a prebuilt binary

#### macOS

Install `unixtime` with the homebrew package manager:
```bash
brew tap schoenenberg/tap
brew install unixtime
```

#### cargo package manager

`unixtime` can be installed using the Rust package manager *cargo*:
```bash
cargo install unixtime
```

Then you should be able to launch `unixtime`.

## Benchmark
Benchmarks were executed with the [hyperfine](https://github.com/sharkdp/hyperfine) utility:

```shell
$> hyperfine 'date +%s' 'unixtime' --warmup 50
Benchmark #1: date +%s
  Time (mean ± σ):       1.5 ms ±   0.6 ms    [User: 0.5 ms, System: 0.6 ms]
  Range (min … max):     0.9 ms …   5.5 ms    626 runs
 
  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark #2: unixtime
  Time (mean ± σ):       2.7 ms ±   0.7 ms    [User: 0.9 ms, System: 0.6 ms]
  Range (min … max):     2.0 ms …   6.4 ms    476 runs
 
  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet PC without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Summary
  'date +%s' ran
    1.79 ± 0.85 times faster than 'unixtime'
```
