#[macro_use]
extern crate clap;

use std::io::{self, BufRead, Write};
use std::{thread, time};

use clap::{App, Arg};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("LPS")
                .help("Rate of lines per second to output (default is 10)")
                .short("r")
                .takes_value(true),
        )
        .get_matches();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let lines_per_second = value_t!(matches, "LPS", u64).unwrap_or(10);
    let gap_millis = time::Duration::from_millis(1000 / lines_per_second);
    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_line(&mut line) {
        if n_bytes == 0 {
            break;
        }

        write!(stdout, "{}", line)?;
        line.clear();
        thread::sleep(gap_millis);
    }

    Ok(())
}
