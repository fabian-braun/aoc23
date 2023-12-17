use itertools::Itertools;
use maplit::hashmap;
use std::fmt;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;

type GameId = usize;

fn main() {
    // part I
    let game_id_sum: usize = read_to_string("input_day2")
        .unwrap()
        .lines()
        .map(String::from)
        .filter_map(|s| {
            let (id, draws) = split_line(s);
            let possible = draws_possible(&draws);
            possible.then_some(id)
        })
        .sum();
    println!("Hello, world! The sum of Game IDs is {}", game_id_sum);
    // part II
    let power_sum: usize = read_to_string("input_day2")
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| {
            let (id, draws) = split_line(s);
            let hull = draws_hull(&draws);
            let power = hull.power();
            println!("Game {}, power {}, hull {:?}", id, power, hull);
            power
        })
        .sum();
    println!("The power of all games is {}", power_sum);
}

#[derive(Debug, Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn draws_possible(draws: &[Draw]) -> bool {
    draws
        .iter()
        .all(|draw| draw.red < 13 && draw.green < 14 && draw.blue < 15)
}

fn draws_hull(draws: &[Draw]) -> Draw {
    draws.iter().fold(Draw::default(), |acc, x| Draw {
        red: acc.red.max(x.red),
        green: acc.green.max(x.green),
        blue: acc.blue.max(x.blue),
    })
}

fn split_line(line: String) -> (GameId, Vec<Draw>) {
    let (game_id, game) = line.split_once(':').unwrap();
    let (_, game_id) = game_id.split_once(' ').unwrap();
    let game_id = GameId::from_str(game_id).unwrap();

    let game: String = game.split_whitespace().collect();
    let draws = game.split(';');
    let draws = draws
        .map(|draw| {
            let parts = draw.split(',');
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for part in parts {
                if let Some(prefix) = part.strip_suffix("red") {
                    red = usize::from_str(prefix).unwrap();
                }
                if let Some(prefix) = part.strip_suffix("green") {
                    green = usize::from_str(prefix).unwrap();
                }
                if let Some(prefix) = part.strip_suffix("blue") {
                    blue = usize::from_str(prefix).unwrap();
                }
            }
            Draw { red, green, blue }
        })
        .collect_vec();

    (game_id, draws)
}

#[cfg(test)]
mod tests {
    use crate::find_pattern;

    #[test]
    fn test_something() {}
}
