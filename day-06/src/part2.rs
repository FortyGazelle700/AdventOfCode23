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
  let file: std::slice::Iter<'_, String> = file.iter();
  let race: Vec<i64> = file
    .map(|line| {
      line
        .split_whitespace()
        .collect::<Vec<&str>>()[1..]
        .join("").parse::<i64>().unwrap()
    }).collect();
  let result: usize;
  let mut race: std::vec::IntoIter<i64> = race.into_iter();
  let time: i64 = race.next().unwrap();
  let record_distance: i64 = race.next().unwrap();
  let records = get_possible_distances(time).into_iter().filter(|outcome| {
    outcome > &record_distance
  }).collect::<Vec<i64>>().len();
  result = records;
  format!("{}", result)
}

fn get_possible_distances(time: i64) -> Vec<i64> {
  let mut distances = Vec::new();
  for hold_time in 1..time {
    let speed = time - hold_time;
    let distance = speed * hold_time;
    distances.push(distance);
  }
  distances
}

// Code from: https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
// fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
// where
//     T: Clone,
// {
//     assert!(!v.is_empty());
//     (0..v[0].len())
//         .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
//         .collect()
// }
