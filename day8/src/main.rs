use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_context = read_to_string("input_day8").unwrap();
    let policy = file_context
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => true,
            _ => false,
        })
        .collect_vec();

    let mut map = HashMap::<&str, (&str, &str)>::new();
    file_context.lines().skip(2).for_each(|line| {
        let (src, left_right) = line.split_once(" = (").unwrap();
        let (left, right) = left_right
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        map.insert(src, (left, right));
    });
    let mut current = "AAA";
    let mut policy_index = 0usize;
    let mut steps = 0usize;
    while current != "ZZZ" {
        let (left, right) = &map[&current];
        current = if policy[policy_index] { left } else { right };
        policy_index += 1;
        policy_index %= policy.len();
        steps += 1;
    }

    println!("Part I solution: {}", steps);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
