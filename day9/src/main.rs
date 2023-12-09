use std::fs::read_to_string;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let file_context = read_to_string("input_day9").unwrap();
    let (part_1, part_2): (Vec<i64>, Vec<i64>) = file_context.lines()
        .map(|line| {
            let seq = line.split(' ').map(|num_str| i64::from_str(num_str).unwrap()).collect_vec();
            let mut firsts = vec![seq[0]];
            let mut finals = vec![seq[seq.len() - 1]];
            let mut diff = seq.iter().tuple_windows::<(_, _)>().map(|(a, b)| {
                b - a
            }).collect_vec();
            while diff.iter().all_equal_value() != Ok(&0i64) {
                firsts.push(diff[0]);
                finals.push(diff[diff.len() - 1]);
                diff = diff.iter().tuple_windows::<(_, _)>().map(|(a, b)| {
                    b - a
                }).collect_vec();
            }
            let part_1_result = finals.iter().rev().fold(0, |acc, x| {
                acc + x
            });
            let part_2_result = firsts.iter().rev().fold(0, |acc, x| {
                x - acc
            });
            (part_1_result, part_2_result)
        }).unzip();

    println!("Part I solution: {}, Part II solution {}", part_1.iter().sum::<i64>(), part_2.iter().sum::<i64>());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}