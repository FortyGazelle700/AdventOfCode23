// 148041808 -- YES (in 45m)

use aoc_utils::read_file::read_file;
use std::thread;
use std::time::{Instant, Duration};


#[derive(Debug,Clone)]
struct MutationItem {
  start: i64,
  end: i64,
  offset: i64,
}

const MAX_THREAD_COUNT: usize = 128;

pub fn main() {
  let now: Instant = Instant::now();
  let file: Vec<String> = read_file(false, None);
  let answer: String = code(file);
  let elapsed: Duration = now.elapsed();
  println!("ANSWER: {answer}");
  println!("ELAPSED: {:.2?}", elapsed);
}

fn code(file: Vec<String>) -> String {
  println!("Code Start");
  let mut file: std::slice::Iter<'_, String> = file.iter();
  let mut seeds = file
    .next()
    .unwrap()
    .split(": ")
    .nth(1)
    .unwrap_or("")
    .split_whitespace()
    .map(|num| {num.parse::<i64>().unwrap()});
  let mut mutated_values: Vec<i64> = Vec::new();
  println!("Getting Seeds");
  loop {
    let start = seeds.next().unwrap_or(-1);
    let length = seeds.next().unwrap_or(-1);
    if start == -1 || length == -1 {break}
    for seed in start..start+length-1 {
      mutated_values.push(seed);
    }
  }
  file.next();
  file.next();
  let mut mutations: Vec<Vec<MutationItem>> = Vec::new();
  let mut current_line: String;
  let mut current_mutation: Vec<MutationItem> = Vec::new();
  println!("Getting Mutations Required");
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
    let dest_start  : i64 = data.next().unwrap().parse::<i64>().unwrap();
    let source_start: i64 = data.next().unwrap().parse::<i64>().unwrap();
    let source_len  : i64 = data.next().unwrap().parse::<i64>().unwrap();
    current_mutation.push(MutationItem {
      start: source_start,
      end: source_start + source_len - 1,
      offset: dest_start - source_start,
    });
  }
  println!("Creating threads");
  let thread_iters = &mutated_values.len() / MAX_THREAD_COUNT;
  println!("Iters: {}", thread_iters);
  let mut comp_time_vec: Vec<Duration> = Vec::new();
  let mut avg_time: u128 = 0;
  for thread_iter in 0..thread_iters+1 {
    let start: Instant = Instant::now();
    let approx_time_remaing = Duration::from_millis((avg_time * u128::try_from(thread_iters - thread_iter).unwrap()).try_into().unwrap());
    
    let secs_remaining: i64 = i64::try_from(approx_time_remaing.as_secs()).unwrap();
    let mins_remaining: i64 = secs_remaining / 60;
    let hrs_remaining: i64 = mins_remaining / 60;
    let mins_remaining: i64 = mins_remaining - (hrs_remaining * 60);
    let secs_remaining: i64 = secs_remaining - (mins_remaining * 120);
    println!("Thread Iteration {:0>9} / {:0>9} (avg. time: {}ms) (approx. time remaining: {}h {}m {}s)", thread_iter, thread_iters, avg_time, hrs_remaining, mins_remaining, secs_remaining);
    let mut threads = Vec::new();
    // for value in &mut mutated_values {
    for idx in 0..MAX_THREAD_COUNT {
      let mutations: std::vec::IntoIter<Vec<MutationItem>> = mutations.clone().into_iter();
      if &idx >= &mutated_values.len() { break };
      let mut value: i64 = mutated_values[idx];
      let thread: thread::JoinHandle<(i64,usize)> = thread::spawn(move || {
        mutations.for_each(|mutation_list: Vec<MutationItem>| {
          if let Some(correct_mutation) = mutation_list.iter().find(|mutation: &&MutationItem| {value >= i64::from(mutation.start) && value <= i64::from(mutation.end)}) {
            value += correct_mutation.offset;
          }
        });
        (value,idx+(thread_iter*MAX_THREAD_COUNT))
      });
      threads.push(thread);
    }

    for thread in threads {
      let value: (i64, usize) = thread.join().unwrap();
      mutated_values[value.1] = value.0;
    }
    comp_time_vec.push(start.elapsed());
    comp_time_vec = comp_time_vec.clone().into_iter().rev().take(120).rev().collect();
    avg_time = comp_time_vec.clone().into_iter().map(|dur| dur.as_millis()).reduce(|a,b| {a+b}).unwrap() / u128::try_from(comp_time_vec.len()).unwrap();
  }
  println!("{:#?}",mutated_values);
  let answer: i64 = mutated_values.into_iter().min().unwrap_or(-1);
  format!("{}", answer)
}