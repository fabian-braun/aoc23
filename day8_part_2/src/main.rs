use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::Itertools;
use bitvec::prelude::*;

fn main() {
    let file_context = read_to_string("example_day8_part2").unwrap();
    let policy = file_context.lines().next().unwrap().chars().map(|c| {
        match c {
            'L' => { true }
            _ => { false }
        }
    }).collect_vec();

    let mut str_to_idx = HashMap::<String, usize>::new();
    let mut idx_to_str = Vec::<String>::new();
    file_context.lines()
        .skip(2)
        .for_each(|line| {
            let (src, _left_right) = line.split_once(" = (").unwrap();
            if !str_to_idx.contains_key(src) {
                str_to_idx.insert(src.to_string(), idx_to_str.len());
                idx_to_str.push(src.to_string());
            }
        });
    let mut current: BitVec<usize, Lsb0> = BitVec::new();
    let mut terminal: BitVec<usize, Lsb0> = BitVec::new();
    let mut left_of: Vec<usize> = Vec::new();
    let mut right_of: Vec<usize> = Vec::new();
    idx_to_str.iter().enumerate().for_each(|(idx, s)| {
        current.push(s.ends_with('A'));
        terminal.push(!s.ends_with('Z'));
        left_of.push(idx);
        right_of.push(idx);
    });
    file_context.lines()
        .skip(2)
        .for_each(|line| {
            let (src, left_right) = line.split_once(" = (").unwrap();
            let (left, right) = left_right.strip_suffix(')').unwrap().split_once(", ").unwrap();
            let src = str_to_idx[src];
            let left = str_to_idx[left];
            let right = str_to_idx[right];
            left_of[src] = left;
            right_of[src] = right;
        });
    let mut policy_index = 0usize;
    let mut steps = 0usize;
    println!("terminal {}", terminal);
    println!("current  {}", current);
    while !(current.clone() | &terminal).all() {
        let idxs = current.iter_ones().collect_vec();
        if policy[policy_index] {
            //left
            for idx in idxs {
                current.swap(idx, left_of[idx])
            }
        } else {
            //right
            for idx in idxs {
                current.swap(idx, right_of[idx])
            }
        };
        policy_index += 1;
        policy_index %= policy.len();
        steps += 1;
        println!("current  {}", current);
    }

    println!("Part II solution: {}", steps);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}