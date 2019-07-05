use std::{thread, time};
use std::io::{self, BufRead, Write};

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let LPS = 100;
    let gap_millis = time::Duration::from_millis(1000 / LPS);

    for line in stdin.lock().lines() {
        writeln!(handle, "{}", line.unwrap())?;
        thread::sleep(gap_millis);
    }

    Ok(())
}
