use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::dict;
use crate::wordle;

pub fn contains_filter(guess: &String, results: &Vec<wordle::Result>) -> HashMap::<char, bool> {
    let mut idx = 0;
    /*
    guess.chars().map(|c| -> (char, bool) {
        idx += 1;
        (c, results[idx - 1] != wordle::Result::GREY)
    }).into_iter().collect()
    */

    let mut filter = HashMap::<char, bool>::new();
    guess.chars().for_each(|c| {
        let not_grey = results[idx] != wordle::Result::GREY;
        filter.entry(c).and_modify(|b| *b = *b || not_grey).or_insert(not_grey);
        idx += 1;
    });
    filter
}

pub fn join_contains_filters(filter1: &HashMap::<char, bool>, filter2: &HashMap::<char, bool>) -> HashMap::<char, bool> {
    let mut filter = HashMap::<char, bool>::new();
    filter1.iter().for_each(|(c, expected)| {
        filter.entry(*c).and_modify(|b| *b = *b || *expected).or_insert(*expected);
    });
    filter2.iter().for_each(|(c, expected)| {
        filter.entry(*c).and_modify(|b| *b = *b || *expected).or_insert(*expected);
    });
    filter
}

pub fn position_filter(guess: &String, results: &Vec<wordle::Result>) -> Vec<HashMap::<char, bool>> {
    let mut idx = 0;
    guess.chars().map(|c| -> HashMap::<char, bool> {
        idx += 1;
        HashMap::<char, bool>::from_iter(IntoIter::new([(c, results[idx - 1] == wordle::Result::GREEN)]))
    }).collect::<Vec<HashMap::<char, bool>>>()
}

pub fn join_position_filters(filter1: &Vec<HashMap::<char, bool>>, filter2: &Vec<HashMap::<char, bool>>) -> Vec<HashMap::<char, bool>> {
    filter1.iter().zip(filter2.iter()).collect::<Vec<(&HashMap::<char, bool>, &HashMap::<char, bool>)>>().iter().map(|(map1, map2): &(&HashMap::<char, bool>, &HashMap::<char, bool>)| -> HashMap::<char, bool> {
        let mut map = HashMap::<char, bool>::new();
        map.extend(map1.into_iter());
        map.extend(map2.into_iter());
        map
    }).collect::<Vec<HashMap::<char, bool>>>()
}

pub fn matches_filters(word: &str, contains_filter: &HashMap::<char, bool>, position_filter: &Vec<HashMap::<char, bool>>) -> bool {
    let mut idx = 0;
    !contains_filter.iter().map(|(c, expected): (&char, &bool)| -> bool {
        word.contains(*c) == *expected
    }).into_iter().any(|b| !b) && !position_filter.iter().map(|filter_map| -> bool {
        idx += 1;
        !filter_map.iter().map(|(c, expected)| -> bool {
            (word.chars().nth(idx - 1) == Some(*c)) == *expected
        }).into_iter().any(|b| !b)
    }).into_iter().any(|b| !b)
}

pub fn iter_solve(solution: &str) -> Vec<String> {
    let wordbank = dict::best_words();
    let mut guess_idx = 0;
    let mut tries = 0;
    let mut c_filter: HashMap::<char, bool> = HashMap::new();
    let mut p_filter: Vec<HashMap::<char, bool>> = vec![HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];
    let mut results = vec![wordbank[guess_idx].clone()];
    while wordbank[guess_idx] != solution && tries < 15 {
        let result = wordle::check(&wordbank[guess_idx].to_string(), &solution.to_string());
        c_filter = join_contains_filters(&c_filter, &contains_filter(&wordbank[guess_idx].to_string(), &result));
        p_filter = join_position_filters(&p_filter, &position_filter(&wordbank[guess_idx].to_string(), &result));
        while !matches_filters(&wordbank[guess_idx], &c_filter, &p_filter) && guess_idx < wordbank.len() - 1 {
            guess_idx += 1;
        }
        results.push(wordbank[guess_idx].to_string());
        tries += 1;
    }
    results
}
