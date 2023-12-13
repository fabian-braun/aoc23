use ndarray::Array2;

#[tokio::main]
async fn main() {
    let content = utilities::get_input(10).await;
    let y_len: usize = content.lines().count();
    let x_len: usize = content.lines().next().unwrap().len();
    let mut map = Array2::from_elem((y_len, x_len), '.');

    let mut start_y = 0;
    let mut start_x = 0;

    content.lines()
        .enumerate().for_each(|(y, line)| {
        line.chars().enumerate()
            .for_each(|(x, c)| {
                map[(y, x)] = c;
                if c == 'S' {
                    start_y = y;
                    start_x = x;
                }
            });
    });

    println!("Part I solution: {}", map);
}

// prev, curr, next
fn next_y_x(curr_c: char, (prev_y, prev_x): (i64, i64)) -> (i64, i64) {
    match curr_c {
        '|' => {}
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
