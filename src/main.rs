use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let numbers = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let calibration_value_sum: usize = read_to_string("input_day1")
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| {
            let i_first = s.find(numbers).unwrap();
            let i_last = s.rfind(numbers).unwrap();
            let first = usize::from_str(&s.chars().nth(i_first).unwrap().to_string()).unwrap();
            let last = usize::from_str(&s.chars().nth(i_last).unwrap().to_string()).unwrap();
            println!("{first}{last}");
            first + last
        })
        .sum();
    println!("Hello, world! The calibration sum is {}", calibration_value_sum);
}
