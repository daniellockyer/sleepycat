# :cat2: sleepycat

[![Build Status](https://travis-ci.com/daniellockyer/sleepycat.svg?branch=master)](https://travis-ci.com/daniellockyer/sleepycat)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

`sleepycat` takes in data from `stdin` and outputs on `stdout` at a given number of lines per second (LPS).

## Why

I was writing a program to parse a log file and draw graphs in real-time. I needed a way to repeatedly test it without generating log files from the source application. `sleepycat` was born so I could reuse the same log file and pipe it into my application at a customizable rate.

Sure, you could do it in Bash with some clever scripting, but I wanted the ability to add in some custom patterns at some point.

## Installation

You have several ways to get `sleepycat`:

1. Download from [crates.io](https://crates.io/crates/sleepycat): `cargo install sleepycat`
2. \*Coming Soon\* Download a prebuilt binary from the [releases page](https://github.com/daniellockyer/sleepycat/releases).
3. Build it yourself.
    1. Clone this repo: `git clone https://github.com/daniellockyer/sleepycat`
    2. Build the binary in release mode: `cargo build --release`
    3. The binary is available at `target/release/sleepycat`

## Accuracy

Right now, this project is just an attempt to get something running. It doesn't take into account the time it takes to print a line, so it `sleep`s for too long and the actual LPS ends up being lower than the target. It's not so much of a big deal with smaller targets, but it struggles to reach higher numbers.

You can test the accuracy using the following command:

```
cargo run --release < access.log | pv --line-mode --rate > /dev/null
```

* For a target of 10 LPS, it reaches 9.99.
* For a target of 100 LPS, it reaches 99.1.
* For a target of 500 LPS, it reaches 476.
