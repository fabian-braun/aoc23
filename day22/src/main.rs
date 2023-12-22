use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use petgraph::dot::Dot;
use petgraph::{Directed, Graph, Incoming, Outgoing};
use petgraph::prelude::EdgeRef;

#[tokio::main]
async fn main() {
    let content = utilities::get_example(22).await;
    let alphabet = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
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
                id: alphabet[idx % alphabet.len()],
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
    let g = bricks.iter().enumerate().map(|(idx, brick)| {
        let node_idx = graph.add_node(*brick);
        node_idx
    }).collect_vec();
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
    println!("{}", Dot::new(&graph));
    let mut something_fell = true;
    while something_fell {
        let updated_bricks = graph.node_indices().filter_map(|brick_n| {
            let brick = graph[brick_n];
            let mut new_z_min = 0;
            graph.neighbors_directed(brick_n, Incoming).for_each(|supporting_brick_n| {
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
        }).collect_vec();
        something_fell = !updated_bricks.is_empty();
        updated_bricks.into_iter().for_each(|brick| {
            graph[g[brick.idx]] = brick;
        });
    }

    // convert into graph of direct support
    let to_remove = graph.edge_references().filter_map(|edge_ref| {
        let supporting_node_n = edge_ref.source();
        let supported_node_n = edge_ref.target();
        let z_max = graph.node_weight(supporting_node_n).unwrap().z_max;
        let z_min = graph.node_weight(supported_node_n).unwrap().z_max;
        if z_min != z_max + 1 {
            Some(edge_ref.id())
        } else {
            None
        }
    }).collect_vec();

    to_remove.iter().for_each(|to_remove| {
        graph.remove_edge(*to_remove);
    });
    println!("{}", Dot::new(&graph));

    let disintegratable_bricks: usize = graph.node_indices().filter(|brick_n| {
        graph.neighbors_directed(*brick_n, Outgoing).all(|supported_neighbour_n| {
            let supporting_z_max = graph.neighbors_directed(supported_neighbour_n, Incoming).map(|supporting_neighbour_n| {
                graph[supporting_neighbour_n].z_max
            }).max().unwrap_or_default();
            let supporting_neighbours = graph.neighbors_directed(supported_neighbour_n, Incoming).filter(|supporting_neighbour_n| {
                graph[*supporting_neighbour_n].z_max == supporting_z_max
            }).count();
            supporting_neighbours > 1
        })
    }).count();


    println!("Part I solution: {}", disintegratable_bricks);
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
struct Brick {
    // all inclusive
    idx: usize,
    id: char,
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
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}
