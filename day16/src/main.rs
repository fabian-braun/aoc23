use std::collections::HashSet;
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
    content.lines()
        .enumerate().for_each(|(y, line)| {
        line.chars().enumerate()
            .for_each(|(x, c)| {
                map[(y, x)] = c;
            });
    });

    let mut initial_beam_states = vec![];
    for y_start in 0..y_len {
        initial_beam_states.push(BeamState {
            direction: RIGHT,
            y: y_start as i64,
            x: 0,
        });
        initial_beam_states.push(BeamState {
            direction: LEFT,
            y: y_start as i64,
            x: (x_len - 1) as i64,
        });
    }
    for x_start in 0..x_len {
        initial_beam_states.push(BeamState {
            direction: DOWN,
            y: 0,
            x: x_start as i64,
        });
        initial_beam_states.push(BeamState {
            direction: UP,
            y: (y_len - 1) as i64,
            x: x_start as i64,
        });
    }

    let result: usize = initial_beam_states.into_iter().map(|beam_state: BeamState| {
        let mut energized = Array2::from_elem((y_len, x_len), false);
        let mut seen_states = HashSet::new();
        let mut beam_state = vec![beam_state];
        while !beam_state.is_empty() {
            beam_state.iter().for_each(|beam_state| {
                seen_states.insert(beam_state.clone());
            });
            beam_state = beam_state.into_iter().flat_map(|beam_state: BeamState| {
                let x = beam_state.x as usize;
                let y = beam_state.y as usize;
                energized[(y, x)] = true;
                beam_state.transition(map[(y, x)], y_len, x_len)
            }).filter(|beam_state: &BeamState| !seen_states.contains(&beam_state)).collect_vec();
        }
        let result = energized.iter().filter(|x| **x).count();
        println!("intermediate result: {}", result);
        result
    }).max().unwrap_or(0);
    println!("Part II solution: {}", result);
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
