use itertools::Itertools;
use petgraph::dot::Dot;
use petgraph::prelude::EdgeRef;
use petgraph::{Directed, Graph, Incoming, Outgoing};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let content = utilities::get_example(22).await;
    let bricks: Vec<Brick> = content
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (a, b) = line.split_once('~').unwrap();
            let mut a = a.split(',');
            let mut b = b.split(',');
            let x_a = i64::from_str(a.next().unwrap()).unwrap();
            let y_a = i64::from_str(a.next().unwrap()).unwrap();
            let z_a = i64::from_str(a.next().unwrap()).unwrap();
            let x_b = i64::from_str(b.next().unwrap()).unwrap();
            let y_b = i64::from_str(b.next().unwrap()).unwrap();
            let z_b = i64::from_str(b.next().unwrap()).unwrap();

            Brick {
                idx,
                z_min: z_a.min(z_b),
                z_max: z_a.max(z_b),
                y_min: y_a.min(y_b),
                y_max: y_a.max(y_b),
                x_min: x_a.min(x_b),
                x_max: x_a.max(x_b),
            }
        })
        .collect_vec();
    let mut graph: Graph<Brick, bool, Directed> = Graph::new();
    let g = bricks
        .iter()
        .enumerate()
        .map(|(idx, brick)| {
            let node_idx = graph.add_node(*brick);
            node_idx
        })
        .collect_vec();
    bricks.iter().combinations(2).for_each(|brickpair| {
        let a = brickpair[0];
        let b = brickpair[1];
        if a.is_below(b) {
            graph.add_edge(g[a.idx], g[b.idx], true);
        }
        if b.is_below(a) {
            graph.add_edge(g[b.idx], g[a.idx], true);
        }
    });
    // println!("{}", Dot::new(&graph));
    let mut something_fell = true;
    while something_fell {
        let updated_bricks = graph
            .node_indices()
            .filter_map(|brick_n| {
                let brick = graph[brick_n];
                let mut new_z_min = 1;
                graph
                    .neighbors_directed(brick_n, Incoming)
                    .for_each(|supporting_brick_n| {
                        let supporting_brick = graph[supporting_brick_n];
                        new_z_min = new_z_min.max(supporting_brick.z_max + 1)
                    });
                if brick.z_min != new_z_min {
                    Some(Brick {
                        z_min: new_z_min,
                        z_max: new_z_min + brick.z_max - brick.z_min,
                        ..brick
                    })
                } else {
                    None
                }
            })
            .collect_vec();
        something_fell = !updated_bricks.is_empty();
        updated_bricks.into_iter().for_each(|brick| {
            graph[g[brick.idx]] = brick;
        });
    }

    // convert into graph of direct support
    let to_remove = graph
        .edge_references()
        .filter_map(|edge_ref| {
            let supporting_node_n = edge_ref.source();
            let supported_node_n = edge_ref.target();
            let z_max = graph.node_weight(supporting_node_n).unwrap().z_max;
            let z_min = graph.node_weight(supported_node_n).unwrap().z_min;
            if z_min != z_max + 1 {
                Some(edge_ref.id())
            } else {
                None
            }
        })
        .collect_vec();

    to_remove.iter().rev().for_each(|to_remove| {
        assert!(graph.remove_edge(*to_remove).is_some());
    });

    let others_to_fall = |brick_n| {
        graph.neighbors_directed(brick_n, Outgoing).filter(|nei| {
            let supported_by = graph.neighbors_directed(*nei, Incoming).count();
            supported_by == 1
        })
    };
    let falling: usize = graph
        .node_indices()
        .map(|brick_n| {
            let mut others_to_fall_count = 0;
            let mut others_to_fall_next = vec![brick_n];
            while !others_to_fall_next.is_empty() {
                others_to_fall_next = others_to_fall_next
                    .iter()
                    .flat_map(|on| others_to_fall(*on))
                    .collect_vec();
                others_to_fall_count += others_to_fall_next.len();
            }
            others_to_fall_count
        })
        .sum();

    println!("\nPart II solution: {}", falling);
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
struct Brick {
    // all inclusive
    idx: usize,
    z_min: i64,
    z_max: i64,
    y_min: i64,
    y_max: i64,
    x_min: i64,
    x_max: i64,
}

impl Brick {
    fn is_below(&self, other: &Self) -> bool {
        //   0123445
        // 0   BBBBB
        // 1 AAXAA B
        // 2 A B A B
        // 3 A BBXBB
        // 4 A   A
        // 5 AAAAA
        // a_dim_x = (1,5)
        // b_dim_x = (0,3) // overlap in x direction
        let overlap_in_x = !(self.x_max < other.x_min || other.x_max < self.x_min);
        let overlap_in_y = !(self.y_max < other.y_min || other.y_max < self.y_min);
        self.z_max < other.z_min && overlap_in_x && overlap_in_y
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{} <- {}",
            self.x_min, self.y_min, self.z_min, self.x_max, self.y_max, self.z_max, self.idx
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
