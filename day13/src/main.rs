use ndarray::{s, Array2, Axis};

#[tokio::main]
async fn main() {
    let content = utilities::get_input(13).await;
    let result: usize = content
        .split("\n\n")
        .map(|pattern| {
            let y_len: usize = pattern.lines().count();
            let x_len: usize = pattern.lines().next().unwrap().len();
            let mut map = Array2::from_elem((y_len, x_len), false);
            pattern.lines().enumerate().for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, c)| {
                    map[(y, x)] = c == '#';
                });
            });
            let mut score = find_reflection_rows(map.clone()) * 100;
            if score == 0 {
                map.swap_axes(0, 1);
                score = find_reflection_rows(map);
            }
            score
        })
        .sum();

    println!("Part I solution: {}", result);
}

fn find_reflection_rows(pattern: Array2<bool>) -> usize {
    let y_min: usize = 0;
    let y_max: usize = pattern.len_of(Axis(0)) - 1;
    'outer: for reflection_index in y_min..y_max {
        let elems_per_reflection = (reflection_index + 1).min(y_max - reflection_index);
        let orig_y_start = reflection_index + 1 - elems_per_reflection;
        let mut y_start = orig_y_start;
        let mut y_end = y_start + elems_per_reflection + elems_per_reflection - 1;
        let mut total_diff = 0;
        while y_start < y_end {
            let diff =
                &pattern.slice(s![y_start..=y_start, ..]) ^ &pattern.slice(s![y_end..=y_end, ..]);
            let diff = diff
                .iter()
                .map(|b: &bool| b.then(|| 1).unwrap_or_default())
                .sum::<usize>();
            total_diff += diff;
            if total_diff > 1 {
                continue 'outer;
            }
            y_start += 1;
            y_end -= 1;
        }
        if total_diff == 1 {
            let rows_above = elems_per_reflection + orig_y_start;
            return rows_above;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
