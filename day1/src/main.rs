use std::io::BufRead;
use std::str::FromStr;
use itertools::Itertools;
use maplit::hashmap;
use utilities::get_input;

#[tokio::main]
async fn main() {
    let numbers = hashmap! {
        "0" => "0",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
    };
    let patterns = numbers.keys().cloned().collect_vec();
    let calibration_value_sum: usize = get_input(1).await
        .lines()
        .map(|s| {
            let first = find_pattern(&patterns, &s, false);
            let last = find_pattern(&patterns, &s, true);
            let mut first = numbers[&first.as_ref()].to_string();
            let last = numbers[&last.as_ref()];
            first.push_str(&last);
            usize::from_str(&first).unwrap()
        })
        .sum();
    println!("Hello, world! The calibration sum is {}", calibration_value_sum);
}

fn find_pattern(pattern: &[&str], input: &str, r: bool) -> String {
    let match_iter = pattern.iter().filter_map(|p| {
        let i_start = if r {
            input.rfind(p)
        } else {
            input.find(p)
        };
        i_start.map(|i_start| {
            (i_start, i_start + p.len())
        })
    }
    );
    let (start, end) = if r {
        match_iter.max_by_key(|tuple| tuple.1).unwrap()
    } else {
        match_iter.min_by_key(|tuple| tuple.0).unwrap()
    };
    let (prefix, _) = input.split_at(end);
    let (_, slice) = prefix.split_at(start);
    slice.to_string()
}


#[cfg(test)]
mod tests {
    use crate::find_pattern;

    #[test]
    fn test_find_pattern() {
        let expected = "la".to_string();
        let actual = find_pattern(&["ab", "la"], "blabla", false);
        assert_eq!(expected, actual);

        let expected = "la".to_string();
        let actual = find_pattern(&["ab", "la"], "blabla", true);
        assert_eq!(expected, actual);

        let expected = "ab".to_string();
        let actual = find_pattern(&["ab", "la"], "blabl", true);
        assert_eq!(expected, actual);

        let expected = "la".to_string();
        let actual = find_pattern(&["ab", "la"], "blabl", false);
        assert_eq!(expected, actual);
    }
}