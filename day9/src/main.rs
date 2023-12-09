use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let file_context = read_to_string("example_day9").unwrap();
    let result: i64 = file_context.lines()
        .map(|line| {
            let seq = line.split(' ').map(|num_str| i64::from_str(num_str).unwrap()).collect_vec();
            let mut finals = vec![seq[seq.len() - 1]];
            let mut diff = seq.iter().tuple_windows::<(_, _)>().map(|(a, b)| {
                b - a
            }).collect_vec();
            while diff.iter().all_equal_value() != Ok(&0i64) {
                finals.push(diff[diff.len() - 1]);
                diff = diff.iter().tuple_windows::<(_, _)>().map(|(a, b)| {
                    b - a
                }).collect_vec();
            }
            finals.iter().rev().fold(0, |acc, x| {
                acc + x
            })
        }).sum();

    println!("Part I solution: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}