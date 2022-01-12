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

## Usage

Print the current Unix timestamp:
```
> unixtime
164202621
```

Print the current timestamp as nanoseconds since January 1, 1970:
```
> unixtime --nanos
1642026460849506000
```

Convert Unix timestamp to human readable format:
```
> unixtime --from=secs 164202621 --rfc2822
Sun, 16 Mar 1975 11:50:21 +0000
> unixtime --from=nanos 1642026460849506000 --rfc3339
2022-01-12T22:27:40.849506+00:00
```

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
