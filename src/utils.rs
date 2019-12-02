use std::fs::File;
use std::io::{BufRead, BufReader, Error, prelude::*};

#[allow(dead_code)]
pub(crate) fn read_lines(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().map(|line| line.unwrap()).collect();
    Ok(lines)
}

#[allow(dead_code)]
pub(crate) fn read_entire_file(path: &str) -> Result<String, Error> {
    let mut input = File::open(path)?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;
    Ok(contents)
}
