use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

type GameId = usize;

fn main() {
    let file_context = read_to_string("input_day4").unwrap();
    let initial_card_count = file_context.lines().count();
    let mut card_counts = vec![1usize; initial_card_count];
    file_context.lines().enumerate().for_each(|(idx, line)| {
        let multiplier = card_counts[idx];
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, draw) = line.split_once(" | ").unwrap();
        let winning: HashSet<usize> = winning
            .split_whitespace()
            .map(usize::from_str)
            .filter_map(Result::ok)
            .collect();
        let mut draw: HashSet<usize> = draw
            .split_whitespace()
            .map(usize::from_str)
            .filter_map(Result::ok)
            .collect();
        draw.retain(|x| winning.contains(x));
        let drawn_winning_count = draw.len();
        for idx in (idx + 1).min(initial_card_count)
            ..(idx + drawn_winning_count + 1).min(initial_card_count)
        {
            card_counts[idx] += multiplier
        }
    });
    let sum: usize = card_counts.iter().sum();
    println!("Part II: The sum is {}", sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
