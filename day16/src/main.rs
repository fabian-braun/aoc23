use itertools::Itertools;
use maplit::hashmap;
use ndarray::Array2;
use Direction::{DOWN, LEFT, RIGHT};
use crate::Direction::UP;


#[tokio::main]
async fn main() {
    let content = utilities::get_input(16).await;
    let y_len: usize = content.lines().count();
    let x_len: usize = content.lines().next().unwrap().len();
    let mut map = Array2::from_elem((y_len, x_len), '.');
    let mut energized = hashmap! {
        UP => Array2::from_elem((y_len, x_len), false),
        DOWN => Array2::from_elem((y_len, x_len), false),
        LEFT => Array2::from_elem((y_len, x_len), false),
        RIGHT => Array2::from_elem((y_len, x_len), false),
    };
    let mut prev_energized = energized.clone();
    content.lines()
        .enumerate().for_each(|(y, line)| {
        line.chars().enumerate()
            .for_each(|(x, c)| {
                map[(y, x)] = c;
            });
    });
    let mut beam_state = vec![BeamState {
        direction: RIGHT,
        y: 0,
        x: 0,
    }];
    let mut initial = true;
    while energized.clone() != prev_energized.clone() || initial {
        initial = false;
        prev_energized = energized.clone();
        beam_state = beam_state.into_iter().flat_map(|beam_state: BeamState| {
            let x = beam_state.x as usize;
            let y = beam_state.y as usize;
            let direction = beam_state.direction;
            energized.get_mut(&direction).unwrap()[(y, x)] = true;
            beam_state.transition(map[(y, x)], y_len, x_len)
        }).collect_vec();
    }
    let result: usize = energized.values().fold(Array2::from_elem((y_len, x_len), false), |acc, x| {
        &acc | x
    }).iter().filter(|x| **x).count();

    println!("Part I solution: {}", result);
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
enum Direction {
    #[default]
    RIGHT,
    LEFT,
    DOWN,
    UP,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
struct BeamState {
    direction: Direction,
    y: i64,
    x: i64,
}

impl BeamState {
    fn transition(self, c: char, y_len: usize, x_len: usize) -> Vec<BeamState> {
        let next = match c {
            '.' => {
                vec![BeamState {
                    direction: self.direction,
                    y: match self.direction {
                        DOWN => { self.y + 1 }
                        UP => { self.y - 1 }
                        _ => { self.y }
                    },
                    x: match self.direction {
                        RIGHT => { self.x + 1 }
                        LEFT => { self.x - 1 }
                        _ => { self.x }
                    },
                }]
            }
            '|' => {
                match self.direction {
                    RIGHT | LEFT => {
                        vec![
                            BeamState {
                                direction: UP,
                                y: self.y - 1,
                                x: self.x,
                            },
                            BeamState {
                                direction: DOWN,
                                y: self.y + 1,
                                x: self.x,
                            },
                        ]
                    }
                    DOWN => {
                        vec![
                            BeamState {
                                direction: DOWN,
                                y: self.y + 1,
                                x: self.x,
                            },
                        ]
                    }
                    UP => {
                        vec![
                            BeamState {
                                direction: UP,
                                y: self.y - 1,
                                x: self.x,
                            }]
                    }
                }
            }
            '-' => {
                match self.direction {
                    UP | DOWN => {
                        vec![
                            BeamState {
                                direction: RIGHT,
                                y: self.y,
                                x: self.x + 1,
                            },
                            BeamState {
                                direction: LEFT,
                                y: self.y,
                                x: self.x - 1,
                            },
                        ]
                    }
                    LEFT => {
                        vec![
                            BeamState {
                                direction: LEFT,
                                y: self.y,
                                x: self.x - 1,
                            },
                        ]
                    }
                    RIGHT => {
                        vec![
                            BeamState {
                                direction: RIGHT,
                                y: self.y,
                                x: self.x + 1,
                            }
                        ]
                    }
                }
            }
            '\\' => {
                match self.direction {
                    DOWN => {
                        vec![
                            BeamState {
                                direction: RIGHT,
                                y: self.y,
                                x: self.x + 1,
                            },
                        ]
                    }
                    UP => {
                        vec![
                            BeamState {
                                direction: LEFT,
                                y: self.y,
                                x: self.x - 1,
                            },
                        ]
                    }
                    LEFT => {
                        vec![
                            BeamState {
                                direction: UP,
                                y: self.y - 1,
                                x: self.x,
                            },
                        ]
                    }
                    RIGHT => {
                        vec![
                            BeamState {
                                direction: DOWN,
                                y: self.y + 1,
                                x: self.x,
                            }
                        ]
                    }
                }
            }
            '/' => {
                match self.direction {
                    DOWN => {
                        vec![
                            BeamState {
                                direction: LEFT,
                                y: self.y,
                                x: self.x - 1,
                            },
                        ]
                    }
                    UP => {
                        vec![
                            BeamState {
                                direction: RIGHT,
                                y: self.y,
                                x: self.x + 1,
                            },
                        ]
                    }
                    LEFT => {
                        vec![
                            BeamState {
                                direction: DOWN,
                                y: self.y + 1,
                                x: self.x,
                            },
                        ]
                    }
                    RIGHT => {
                        vec![
                            BeamState {
                                direction: UP,
                                y: self.y - 1,
                                x: self.x,
                            }
                        ]
                    }
                }
            }
            c => { panic!("AAH {c}") }
        };
        next.into_iter().filter(|s| {
            s.y < y_len as i64 && s.y >= 0 &&
                s.x < x_len as i64 && s.x >= 0
        }).collect_vec()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
