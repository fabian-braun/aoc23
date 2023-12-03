use std::fmt;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;
use itertools::Itertools;
use maplit::hashmap;

type GameId = usize;

fn main() {
    let numbers = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let specials = &['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let file_context = read_to_string("example_day3").unwrap();
    let y_len = file_context.lines().count();
    let x_len = file_context.lines().next().unwrap().len();
    let mut symbol_mask = bit_matrix::BitMatrix::new(y_len, x_len);
    file_context.lines().enumerate()
        .for_each(|(y, line)| {
            let mut x = 0usize;
            line.chars().for_each(|c| {
                if !specials.contains(&c) {
                    // we found a symbol
                    for yy in y.saturating_sub(1)..=(y + 1).min(y_len - 1) {
                        for xx in x.saturating_sub(1)..=(x + 1).min(x_len - 1) {
                            symbol_mask.set(yy, xx, true);
                        }
                    }
                }
                x += 1;
            })
        });

    println!("Part I: The sum is {}", 3);
    println!("Part II: The sum is {}", 3);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}