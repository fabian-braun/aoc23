use ndarray::{Array2, Axis};

#[tokio::main]
async fn main() {
    let content = utilities::get_input(13).await;
    let result: usize = content.split("\n\n").map(|pattern| {
        let y_len: usize = pattern.lines().count();
        let x_len: usize = pattern.lines().next().unwrap().len();
        let mut map = Array2::from_elem((y_len, x_len), '.');
        pattern.lines()
            .enumerate().for_each(|(y, line)| {
            line.chars().enumerate()
                .for_each(|(x, c)| {
                    map[(y, x)] = c;
                });
        });
        let mut score = find_reflection_rows(map.clone()) * 100;
        if score == 0 {
            map.swap_axes(0, 1);
            score = find_reflection_rows(map);
        }
        score
    }).sum();

    println!("Part I solution: {}", result);
}

fn find_reflection_rows(pattern: Array2<char>) -> usize {
    println!("{}", pattern);
    let y_min: usize = 0;
    let y_max: usize = pattern.len_of(Axis(0)) - 1;
    'outer: for reflection_index in y_min..y_max {
        let elems_per_reflection = (reflection_index + 1).min(y_max - reflection_index);
        let orig_y_start = reflection_index + 1 - elems_per_reflection;
        let mut y_start = orig_y_start;
        let mut y_end = y_start + elems_per_reflection + elems_per_reflection - 1;
        println!("elems per reflection {} in {},{}", elems_per_reflection, y_start, y_end);
        while y_start < y_end {
            println!("{} == {}", pattern.row(y_start), pattern.row(y_end));
            if pattern.row(y_start) != pattern.row(y_end) {
                continue 'outer;
            }
            y_start += 1;
            y_end -= 1;
        }
        let rows_above = elems_per_reflection + orig_y_start;
        println!("Found reflection {}", rows_above);
        return rows_above;
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
