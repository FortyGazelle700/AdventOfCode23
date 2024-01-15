use std::{fs::File, io::{self, BufRead}};

pub fn read_file(use_ex: bool, max_lines: Option<usize>) -> Vec<String> {
    let file_name: &str = if use_ex {"data/ex.txt"} else {"data/ds.txt"};
    let file: File = File::open(file_name).unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .take(max_lines.unwrap_or(usize::MAX))
        .map(|line: Result<String, io::Error>| {line.unwrap()})
        .collect();
    lines
}