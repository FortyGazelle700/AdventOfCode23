// 214922730 -- YES YES YES

use aoc_utils::read_file::read_file;
use std::time::{Instant, Duration};

#[derive(Debug,Clone)]
struct MutationItem {
  start: i64,
  end: i64,
  offset: i64,
}

pub fn main() {
  let now: Instant = Instant::now();
  let file: Vec<String> = read_file(false, None);
  let answer = code(file);
  let elapsed: Duration = now.elapsed();
  println!("ANSWER: {answer}");
  println!("ELAPSED: {:.2?}", elapsed);
}

fn code(file: Vec<String>) -> String {
  let mut file: std::slice::Iter<'_, String> = file.iter();
  let mut mutated_values:Vec<i64> = file
    .next()
    .unwrap()
    .split(": ")
    .nth(1)
    .unwrap_or("")
    .split_whitespace()
    .map(|num| {num.parse::<i64>().unwrap()})
    .collect();
  file.next();
  file.next();
  let mut mutations: Vec<Vec<MutationItem>> = Vec::new();
  let mut current_line: String;
  let mut current_mutation: Vec<MutationItem> = Vec::new();
  loop {
    current_line = file.next().unwrap_or(&String::from("EOF")).to_string();
    if current_line == "" {continue;}
    if current_line.contains(":") || current_line == "EOF" {
      mutations.push(current_mutation.to_vec());
      current_mutation = Vec::new();
      if current_line == "EOF" {break;}
      continue;
    }
    let mut data: std::str::SplitWhitespace<'_> = current_line.split_whitespace();
    let dest_start: i64 =   data.next().unwrap().parse::<i64>().unwrap();
    let source_start: i64 = data.next().unwrap().parse::<i64>().unwrap();
    let source_len: i64 =   data.next().unwrap().parse::<i64>().unwrap();
    current_mutation.push(MutationItem {
      start: source_start,
      end: source_start + source_len - 1,
      offset: dest_start - source_start,
    });
  }
  // println!("MUTATIONS: {:#?}", &mutations);
  mutations
    .into_iter()
    .for_each(|mutation_list: Vec<MutationItem>| {
            mutated_values = mutated_values
        .clone()
        .into_iter()
        .map(|mut value: i64| {
          if let Some(correct_mutation) = mutation_list.iter().find(|mutation: &&MutationItem| {value >= i64::from(mutation.start) && value <= i64::from(mutation.end)}) {
            value += correct_mutation.offset;
          }
          value
        }).collect();
    });
    // println!("ALL VALUES: {:#?}", &mutated_values);
  let answer: i64 = mutated_values.into_iter().min().unwrap();
  format!("{}", answer)
}