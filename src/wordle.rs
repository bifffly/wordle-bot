use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Result {
    GREEN,
    YELLOW,
    GREY
}

pub fn check(guess: &String, solution: &String) -> Vec<Result> {
    let mut idx = 0;
    let mut seen = HashMap::<char, usize>::new();
    guess.chars().map(|c| -> Result {
        idx += 1;
        if solution.chars().nth(idx - 1) == Some(c) {
            seen.entry(c).and_modify(|n| *n += 1).or_insert(1);
            Result::GREEN
        }
        else if solution.contains(c) && seen.entry(c).and_modify(|n| *n += 1).or_insert(1) <= &mut solution.matches(c).count() {
            Result::YELLOW
        }
        else {
            Result::GREY
        }
    }).collect()
}
