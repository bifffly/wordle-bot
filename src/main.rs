use std::collections::HashMap;

use wordle_solver::dict;
use wordle_solver::solver;
use wordle_solver::wordle;

fn main() {
    let mut histogram: HashMap<usize, usize> = HashMap::new();
    let mut wordmap: HashMap<usize, Vec<String>> = HashMap::new();
    dict::SOLS[0..100].iter().for_each(|word| {
        let tries = solver::iter_solve(word).len();
        histogram.entry(tries).and_modify(|n| *n += 1).or_insert(1);
        wordmap.entry(tries).and_modify(|v| v.push(word.to_string())).or_insert(vec![word.to_string()]);
    });
    print!("{:?}\n", histogram);
}
