// 252999085 -- HIGH
// 250749220 -- HIGH
// 250452116 -- LOW
// 250577259 -- YES

use aoc_utils::read_file::read_file;
use std::{time::{Instant, Duration}, cmp::Ordering};

#[derive(Clone,Debug,PartialEq,PartialOrd)]
enum HandType {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPair,
  OnePair,
  HighCard
}

fn convert_hand_to_val(hand_type: HandType) -> u8 {
  match hand_type {
    HandType::FiveOfAKind => 6,
    HandType::FourOfAKind => 5,
    HandType::FullHouse => 4,
    HandType::ThreeOfAKind => 3,
    HandType::TwoPair => 2,
    HandType::OnePair => 1,
    HandType::HighCard => 0,
  }
}

pub fn main() {
  let now: Instant = Instant::now();
  let file: Vec<String> = read_file(false, None);
  let total: String = code(file);
  let elapsed: Duration = now.elapsed();
  println!("Total: {total}");
  println!("Elapsed: {:.2?}", elapsed);
}

fn code(file: Vec<String>) -> String {
  let file: std::vec::IntoIter<String> = file.into_iter();
  let file: Vec<Vec<String>> = file.map(|line: String| {line.split(" ").map(|part: &str| part.to_string()).collect::<Vec<String>>()}).collect();
  let mut filtered_hands: Vec<(String,Vec<u8>,u32)> = file.clone().into_iter().map(|hand: Vec<String>| {
    let mut hand: std::vec::IntoIter<String> = hand.into_iter();
    let cards: String = hand.next().unwrap();
    let priority: Vec<u8> = get_priority(cards.clone());
    let bid: u32 = hand.next().unwrap().parse().unwrap();
    (cards,priority,bid)
  }).collect::<Vec<(String,Vec<u8>,u32)>>();
  filtered_hands.sort_by(|a: &(String,Vec<u8>,u32), b: &(String,Vec<u8>,u32)| {
    let mut a: std::vec::IntoIter<u8> = a.1.clone().into_iter();
    let mut b: std::vec::IntoIter<u8> = b.1.clone().into_iter();
    loop {
      let next_a: u8 = a.next().unwrap_or_else(|| {panic!("identical occurences")});
      let next_b: u8 = b.next().unwrap_or_else(|| {panic!("identical occurences")});
      if next_a > next_b {
        return Ordering::Greater;
      }
      if next_a < next_b {
        return Ordering::Less;
      }
    }
  });
  println!("{}", String::from(format!("{:?}", &filtered_hands)).replace("), (", "),\n("));
  let mut result: i32 = 0;
  filtered_hands.into_iter().enumerate().for_each(|itm: (usize, (String, Vec<u8>, u32))| {
    let multiplier: usize = itm.0 + 1;
    let bid: usize = itm.1.2.try_into().unwrap();
    result += i32::try_from(multiplier * bid).unwrap();
  });
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
    "T"  =>  9,
    "9"  =>  8,
    "8"  =>  7,
    "7"  =>  6,
    "6"  =>  5,
    "5"  =>  4,
    "4"  =>  3,
    "3"  =>  2,
    "2"  =>  1,
    "J"  =>  0,
    _    =>  panic!("Uncaught card {}", card)
  }
}

fn get_hand_type_priority(hand: String) -> u8 {
  let mut cards: Vec<(char, u8)> = Vec::new();
  let mut possible_hand_values: Vec<u8> = Vec::new();
  let immitations_cards: Vec<(char, u8)>;
  hand.chars().for_each(|card: char| {
    let card_count: u8 = hand.chars().filter(|itm: &char| itm == &card).collect::<Vec<char>>().len().try_into().unwrap();
    if cards.iter().find(|itm: &&(char, u8)| itm.0 == card).is_some() {return;}
    cards.push((card, card_count));
  });
  let js: u8 = cards.iter().find(|itm| itm.0 == 'J').unwrap_or(&('J',0)).1;
  if js > 0 {
    immitations_cards = cards.clone().into_iter().filter(|itm| {itm.0 != 'J'}).collect();
    immitations_cards.clone().into_iter().for_each(|possible_card: (char, u8)| {
      let mut cards = immitations_cards.clone();
      cards = cards.into_iter().map(|card: (char, u8)| {
        if card.0 == possible_card.0 {return (card.0,card.1+js)}
        return card;
      }).collect();
      possible_hand_values.push(convert_hand_to_val(find_hand_types(cards)));
    });
  }
  possible_hand_values.push(convert_hand_to_val(find_hand_types(cards)));
  return possible_hand_values.into_iter().max().unwrap_or(0);
}

fn find_hand_types(hand: Vec<(char, u8)>) -> HandType {
  let mut hand_types: Vec<HandType> = Vec::new();
  let mut hand_type: HandType = HandType::HighCard;
  hand.into_iter().for_each(|repeat: (char,u8)| {
    let repeat: u8 = repeat.1;
    match repeat {
      5 => hand_type = HandType::FiveOfAKind,
      4 => hand_type = HandType::FourOfAKind,
      3 => hand_type = HandType::ThreeOfAKind,
      2 => hand_type = HandType::OnePair,
      _ => hand_type = HandType::HighCard
    }
    hand_types.push(hand_type.clone());
  });
  if hand_types.contains(&HandType::FiveOfAKind) {
    hand_type = HandType::FiveOfAKind;
  } else if hand_types.contains(&HandType::FourOfAKind) {
    hand_type = HandType::FourOfAKind;
  } else if hand_types.contains(&HandType::ThreeOfAKind) && hand_types.contains(&HandType::OnePair) {
    hand_type = HandType::FullHouse;
  } else if hand_types.contains(&HandType::ThreeOfAKind) {
    hand_type = HandType::ThreeOfAKind;
  } else if hand_types.clone().into_iter().filter(|itm| itm == &HandType::OnePair).count() == 2 {
    hand_type = HandType::TwoPair;
  } else if hand_types.contains(&HandType::OnePair) {
    hand_type = HandType::OnePair;
  } else {
    hand_type = HandType::HighCard;
  }
  return hand_type;
}