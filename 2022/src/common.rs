use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(file_path: P) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let buffer = BufReader::new(file);
    let lines: Vec<String> = buffer.lines().map(|l| l.expect("no line found")).collect();
    return lines;
}
