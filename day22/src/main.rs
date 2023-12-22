use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use petgraph::dot::Dot;
use petgraph::{Directed, Graph, Incoming, Outgoing};

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
    let something_fell = true;
    while something_fell {

    }
    println!("{}", Dot::new(&graph));

    println!("Part I solution: {}", 0);
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
