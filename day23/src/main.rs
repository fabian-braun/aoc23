use maplit::{hashmap, hashset};
use std::collections::{HashMap, HashSet};

#[tokio::main]
async fn main() {
    let content = utilities::get_input(23).await;

    let mut nodes: HashSet<(i64, i64)> = hashset! {};
    content.lines().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;
            match c {
                '.' => {
                    nodes.insert((y, x));
                }
                '>' => {
                    nodes.insert((y, x));
                }
                'v' => {
                    nodes.insert((y, x));
                }
                '^' => {
                    nodes.insert((y, x));
                }
                '<' => {
                    nodes.insert((y, x));
                }
                _ => {}
            }
        })
    });
    let start = (0 as i64, 1 as i64);
    let end = (
        content.lines().count() as i64 - 1,
        content.lines().next().unwrap().len() as i64 - 2,
    );

    let mut neighbours: HashMap<(i64, i64), Vec<(i64, i64)>> = hashmap! {};
    content.lines().enumerate().for_each(|(y, line)| {
        let y = y as i64;
        line.chars().enumerate().for_each(|(x, c)| {
            let x = x as i64;
            match c {
                '.' => {
                    let candidate_neighbours = [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];
                    for candidate_neighbour in candidate_neighbours {
                        if nodes.contains(&candidate_neighbour) {
                            neighbours
                                .entry((y, x))
                                .and_modify(|x| x.push(candidate_neighbour))
                                .or_insert_with(|| vec![candidate_neighbour]);
                        }
                    }
                }
                '>' => {
                    neighbours.insert((y, x), vec![(y, x + 1)]);
                }
                'v' => {
                    neighbours.insert((y, x), vec![(y + 1, x)]);
                }
                '^' => {
                    neighbours.insert((y, x), vec![(y - 1, x)]);
                }
                '<' => {
                    neighbours.insert((y, x), vec![(y, x - 1)]);
                }
                _ => {}
            }
        })
    });

    let mut visited: HashSet<(i64, i64)> = hashset! {start};
    let mut current_path = vec![start];
    let max = evaluate_neighbours(&mut current_path, &mut visited, &neighbours, &end);

    println!("Part I solution: {}", max - 1);
}

fn evaluate_neighbours(
    path: &mut Vec<(i64, i64)>,
    visited: &mut HashSet<(i64, i64)>,
    neighbours: &HashMap<(i64, i64), Vec<(i64, i64)>>,
    end: &(i64, i64),
) -> usize {
    let current = path.last().unwrap();
    if path.last().unwrap() == end {
        return path.len();
    }

    let mut max = path.len();
    for neighbour in &neighbours[current] {
        if visited.contains(neighbour) {
            continue;
        }
        path.push(*neighbour);
        visited.insert(*neighbour);
        max = max.max(evaluate_neighbours(path, visited, neighbours, end));
        path.pop();
        visited.remove(&neighbour);
    }
    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
