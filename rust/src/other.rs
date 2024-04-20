pub use std::io::{self, prelude::*, stdin};
use std::str::FromStr;

pub fn get_stdin_input<T: FromStr>(prompt: &str) -> io::Result<T> {
    if prompt != "" {
        print!("{prompt}");
    }
    let (stdin, mut line) = (stdin(), String::new());
    loop {
        line.clear();
        stdin.read_line(&mut line)?;
        if let Ok(t) = line.trim().parse() {
            break Ok(t);
        }
    }
}

pub fn get_input<T: FromStr>(prompt: &str, reader: &mut impl BufRead) -> io::Result<T> {
    if prompt != "" {
        print!("{prompt}");
    }
    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line)?;
        if let Ok(t) = line.trim().parse() {
            break Ok(t);
        }
    }
}
