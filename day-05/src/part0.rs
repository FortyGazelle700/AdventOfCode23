use aoc_utils::read_file::read_file;
use std::time::{Instant, Duration};

pub fn main() {
    let now: Instant = Instant::now();
    let file: Vec<String> = read_file(false, None);
    let total = code(file);
    let elapsed: Duration = now.elapsed();
    println!("Total: {total}");
    println!("Elapsed: {:.2?}", elapsed);
}

fn code(file: Vec<String>) -> String {
    let mut result = usize::MAX;
    format!("{result}")
}