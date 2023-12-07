use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let file_context = read_to_string("example_day7").unwrap();
    let count: usize = file_context.lines()
        .map(|line| {

        }).count();

    println!("Part I solution: {}", count);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}