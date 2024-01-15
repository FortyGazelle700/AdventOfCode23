// 251284428 -- LOW
// 252178728 -- LOW
// 252295678 -- YES

use aoc_utils::read_file::read_file;
use std::{time::{Instant, Duration}, cmp::Ordering};

pub fn main() {
  let now: Instant = Instant::now();
  let file: Vec<String> = read_file(false, None);
  let total = code(file);
  let elapsed: Duration = now.elapsed();
  println!("Total: {total}");
  println!("Elapsed: {:.2?}", elapsed);
}

fn code(file: Vec<String>) -> String {
  let file: std::vec::IntoIter<String> = file.into_iter();
  let file: Vec<Vec<String>> = file
    .map(|line: String| {
      line
        .split(" ")
        .map(|part: &str| {part.to_string()})
        .collect::<Vec<String>>()
    })
  .collect();
  let mut filtered_hands: Vec<(String,Vec<u8>,u32)> = file
    .into_iter()
    .map(|hand: Vec<String>| {
      let mut hand: std::vec::IntoIter<String> = hand.into_iter();
      let cards: String = hand.next().unwrap();
      let bid  : u32    = hand.next().unwrap().parse().unwrap();
      let priority: Vec<u8> = get_priority(cards.clone());
      (cards,priority,bid)
    }).collect::<Vec<(String,Vec<u8>,u32)>>();
  filtered_hands.sort_by(|a: &(String,Vec<u8>,u32), b: &(String,Vec<u8>,u32)| {
    let mut a: std::vec::IntoIter<u8> = a.1.clone().into_iter();
    let mut b: std::vec::IntoIter<u8> = b.1.clone().into_iter();
    loop {
      let next_a: u8 = a.next().unwrap_or_else(|| {panic!("rolled over")});
      let next_b: u8 = b.next().unwrap_or_else(|| {panic!("rolled over")});
      if next_a > next_b {
        return Ordering::Greater;
      }
      if next_a < next_b {
        return Ordering::Less;
      }
    }
  });
  // dbg!(filtered_hands.clone().iter().collect::<Vec<_>>());
  let mut result: i32 = 0;
  filtered_hands.into_iter().enumerate().for_each(|itm: (usize, (String, Vec<u8>, u32))| {
    let multiplier: usize = itm.0 + 1;
    let bid: usize = itm.1.2.try_into().unwrap();
    result += i32::try_from(multiplier * bid).unwrap();
  });
  // dbg!(file);
  format!("{result}")
}

fn get_priority(hand: String) -> Vec<u8> {
  let mut priority_vec: Vec<u8> = Vec::new();
  priority_vec.push(get_hand_type_priority(hand.clone()));
  hand.chars().for_each(|card: char| {
    priority_vec.push(get_card_priority(card.to_string()));
  });
  priority_vec
}

fn get_card_priority(card: String) -> u8 {
  return match card.as_str() {
    "A"  => 12,
    "K"  => 11,
    "Q"  => 10,
    "J"  =>  9,
    "T"  =>  8,
    "9"  =>  7,
    "8"  =>  6,
    "7"  =>  5,
    "6"  =>  4,
    "5"  =>  3,
    "4"  =>  2,
    "3"  =>  1,
    "2"  =>  0,
    _    =>  panic!("Uncaught card {}", card)
  }
}

fn get_hand_type_priority(hand: String) -> u8 {
  let mut hand_type: u8 = 0;
  let mut repeats: Vec<(String,usize)> = Vec::new();
  let mut stored_repeat: usize = 1;
  hand.chars().for_each(|char: char| {
    let total_repeat_chars: usize = hand.chars().filter(|new_char: &char| {new_char == &char}).collect::<Vec<char>>().len();
    repeats.push((char.to_string(),total_repeat_chars));
  });
  repeats.into_iter().for_each(|repeat: (String,usize)| {
    let repeat: usize = repeat.1;
    match repeat {
      5 => hand_type = 6,
      4 => hand_type = 5,
      3 => match stored_repeat {
        2 => hand_type = 4,
        _ => hand_type = 3
      },
      2 => match stored_repeat {
        3 => hand_type = 4,
        2 => hand_type = 2,
        _ => hand_type = 1
      },
      _ => {
        if stored_repeat == 1 {hand_type = 0}
      }
    }
    if repeat != 1 {
      stored_repeat = repeat
    }
  });
  hand_type
}