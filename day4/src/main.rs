use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;
use itertools::Itertools;

type GameId = usize;

fn main() {
    let file_context = read_to_string("input_day4").unwrap();
    let sum: usize = file_context.lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (winning, draw) = line.split_once(" | ").unwrap();
            let winning: HashSet<usize> = winning.split_whitespace().map(usize::from_str).filter_map(Result::ok).collect();
            let mut draw: HashSet<usize> = draw.split_whitespace().map(usize::from_str).filter_map(Result::ok).collect();
            draw.retain(|x| { winning.contains(x) });
            let drawn_winning_count = draw.len();
            if drawn_winning_count > 0 {
                2usize.pow((drawn_winning_count - 1) as u32)
            } else {
                0usize
            }
        }).sum();

    println!("Part I: The sum is {}", sum);
    println!("Part II: The sum is {}", 3);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}