# :cat2: sleepycat

[![Build Status](https://travis-ci.com/daniellockyer/sleepycat.svg?branch=master)](https://travis-ci.com/daniellockyer/sleepycat)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

`sleepycat` takes in data from `stdin` and outputs on `stdout` at a given number of lines per second (LPS).

## Why

I was writing a program to parse a log file and draw graphs in real-time. I needed a way to repeatedly test it without generating log files from the source application. `sleepycat` allows you to take a file and output the contents at a configurable number of LPS.

## Accuracy

Right now, this project is just an attempt to get something running. It doesn't take into account the time it takes to print a line, so it `sleep`s for too long and the actual LPS ends up being lower than the target. It's not so much of a big deal with smaller targets, but it struggles to reach higher numbers.

You can test the accuracy using the following command:

```
cargo run --release < access.log | pv --line-mode --rate > /dev/null
```

* For a target of 10 LPS, it reaches 9.99.
* For a target of 100 LPS, it reaches 98.1.
* For a target of 500 LPS, it reaches 460.
