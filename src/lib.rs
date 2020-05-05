mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn calc_next_level_exp(level: u32) -> u32 {
  let base_exp = match level {
    1..=9 => 1_000,
    10..=19 => 2_000,
    20..=29 => 4_100,
    30..=38 => 7200,
    39..=44 => 11_000,
    45..=50 => 18_000,
    51..=54 => 46_500,
    55..=59 => 66_000,
    60 => 0,
    _ => panic!("Invalid level: {}", level),
  };

  let exp_growth = match level {
    1..=9 => 100,
    10..=19 => 200,
    20..=29 => 300,
    30..=38 => 400,
    39..=44 => 1_000,
    45..=50 => 2_000,
    51..=54 => 4_500,
    55..=59 => 6_000,
    60 => 0,
    _ => panic!("Invalid level: {}", level),
  };

  let multiplier = match level {
    1..=9 => level - 1,
    10..=19 => level - 10,
    20..=29 => level - 20,
    30..=38 => level - 30,
    39..=44 => level - 39,
    45..=50 => level - 45,
    51..=54 => level - 51,
    55..=59 => level - 55,
    60 => 0,
    _ => panic!("Invalid level: {}", level),
  };

  base_exp + exp_growth * multiplier
}

#[cfg(test)]
mod calc_next_level_exp {
  use super::*;

  #[test]
  fn level_1() {
    assert_eq!(calc_next_level_exp(1), 1_000);
  }

  #[test]
  fn level_5() {
    assert_eq!(calc_next_level_exp(5), 1_400);
  }

  #[test]
  fn level_10() {
    assert_eq!(calc_next_level_exp(10), 2_000);
  }

  #[test]
  fn level_15() {
    assert_eq!(calc_next_level_exp(15), 3_000);
  }

  #[test]
  fn level_20() {
    assert_eq!(calc_next_level_exp(20), 4_100);
  }

  #[test]
  fn level_25() {
    assert_eq!(calc_next_level_exp(25), 5_600);
  }

  #[test]
  fn level_30() {
    assert_eq!(calc_next_level_exp(30), 7_200);
  }

  #[test]
  fn level_35() {
    assert_eq!(calc_next_level_exp(35), 9_200);
  }

  #[test]
  fn level_40() {
    assert_eq!(calc_next_level_exp(40), 12_000);
  }

  #[test]
  fn level_45() {
    assert_eq!(calc_next_level_exp(45), 18_000);
  }

  #[test]
  fn level_50() {
    assert_eq!(calc_next_level_exp(50), 28_000);
  }

  #[test]
  fn level_51() {
    assert_eq!(calc_next_level_exp(51), 46_500);
  }

  #[test]
  fn level_55() {
    assert_eq!(calc_next_level_exp(55), 66_000);
  }

  #[test]
  fn level_60() {
    assert_eq!(calc_next_level_exp(60), 0);
  }
}
