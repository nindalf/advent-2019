use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[allow(dead_code)]
pub(crate) fn read_lines(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().map(|line| line.unwrap()).collect();
    Ok(lines)
}
