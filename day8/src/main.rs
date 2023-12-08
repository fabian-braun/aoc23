use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let file_context = read_to_string("example_day8").unwrap();
    let result: usize = file_context.lines()
        .map(|line| {
        }).count();

    println!("Part I solution: {}", result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}