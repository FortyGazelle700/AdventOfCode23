use aoc_utils::read_file::read_file;
use std::time::{Instant, Duration};

pub fn main() {
    let now: Instant = Instant::now();

    let file: Vec<String> = read_file(false, None);
    
    let mut lines: Vec<(Vec<i8>, Vec<i8>)> = Vec::new();

    file.iter().for_each(|line: &String| {
        let line: &str = line
            .split(": ")
            .nth(1)
            .unwrap_or("");

        let line: Vec<Vec<i8>> = line
            .split(" | ")
            .map(|half: &str|
                half
                .split_whitespace()
                .map(|number: &str| number
                    .parse::<i8>()
                    .unwrap_or(0)
                )
                .collect::<Vec<i8>>()
            )
            .collect::<Vec<Vec<i8>>>();

        let line: (Vec<i8>, Vec<i8>) = (
            line.clone().into_iter().nth(0).unwrap(),
            line        .into_iter().nth(1).unwrap()
        );
        
        lines.push(line);
    });

    let mut total: i16 = 0;

    lines.iter().for_each(|line: &(Vec<i8>, Vec<i8>)| {
        let mut sub_total: i16 = 0;
        line.0.iter().for_each(|winning_number: &i8| {
            if line.1.contains(winning_number) {
                sub_total += if sub_total == 0 {1} else {sub_total};
            }
        });
        total += sub_total;
    });

    println!("Total: {total}");

    let elapsed: Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}