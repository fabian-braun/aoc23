use bit_matrix::BitMatrix;
use itertools::Itertools;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

type GameId = usize;

fn main() {
    let numbers = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let specials = &['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let file_context = read_to_string("input_day3").unwrap();
    let y_len = file_context.lines().count();
    let x_len = file_context.lines().next().unwrap().len();
    let mut symbol_mask = BitMatrix::new(y_len, x_len);
    file_context.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0usize;
        line.chars().for_each(|c| {
            if !specials.contains(&c) {
                // we found a symbol, enable mask around it
                for yy in y.saturating_sub(1)..=(y + 1).min(y_len - 1) {
                    for xx in x.saturating_sub(1)..=(x + 1).min(x_len - 1) {
                        symbol_mask.set(yy, xx, true);
                    }
                }
            }
            x += 1;
        })
    });

    let mut found_numbers = Vec::<(usize /* y */, usize /* x */, String)>::new();
    file_context.lines().enumerate().for_each(|(y, line)| {
        let mut number_under_construction: Option<(
            usize, /* start_x */
            usize, /* prev_x */
            String,
        )> = None;
        line.match_indices(numbers).for_each(|(x, c)| {
            if let Some((start_x, prev_x, number_str)) = number_under_construction.take() {
                if prev_x + 1 == x {
                    // n-digit number continues
                    number_under_construction = Some((start_x, x, number_str + c));
                } else {
                    // new n-digit number starts, store previous one
                    found_numbers.push((y, start_x, number_str));
                    number_under_construction = Some((x, x, c.to_string()));
                }
            } else {
                number_under_construction = Some((x, x, c.to_string()));
            }
        });
        if let Some((start_x, _, number_str)) = number_under_construction.take() {
            found_numbers.push((y, start_x, number_str));
        }
    });
    println!("{:?}", found_numbers);
    let sum: usize = found_numbers
        .into_iter()
        .filter(|n| eligible(n, &symbol_mask))
        .map(|(_, _, s)| usize::from_str(&s).unwrap())
        .sum();

    println!("Part I: The sum is {}", sum);
    println!("Part II: The sum is {}", 3);
}

fn eligible(number_pos: &(usize, usize, String), mask: &BitMatrix) -> bool {
    let (y, x_start, s) = number_pos;
    let len = s.len();
    for xx in *x_start..(x_start + len) {
        if mask[*y][xx] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
