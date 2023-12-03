use std::collections::HashMap;
use std::fmt::Display;
use std::fs::read_to_string;
use std::str::FromStr;
use bit_matrix::BitMatrix;
use itertools::Itertools;
use maplit::hashmap;
use petgraph::{Graph, graph, Undirected};
use petgraph::data::DataMap;
use petgraph::graph::NodeIndex;
use petgraph::visit::NodeRef;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Default)]
enum Node {
    #[default]
    Edge,
    Num(usize),
    Gear {
        y: usize,
        x: usize,
    },
}


fn main() {
    let numbers = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let file_context = read_to_string("input_day3").unwrap();
    let y_len = file_context.lines().count();
    let x_len = file_context.lines().next().unwrap().len();
    let mut symbol_mask = BitMatrix::new(y_len, x_len);
    file_context.lines().enumerate()
        .for_each(|(y, line)| {
            let mut x = 0usize;
            line.chars().for_each(|c| {
                if c == '*' {
                    symbol_mask.set(y, x, true);
                }
                x += 1;
            })
        });

    let mut found_numbers = Vec::<(usize/* y */, usize /* x */, String)>::new();
    file_context.lines().enumerate()
        .for_each(|(y, line)| {
            let mut number_under_construction: Option<(usize/* start_x */, usize /* prev_x */, String)> = None;
            line.match_indices(numbers).for_each(|(x, c)| {
                if let Some((start_x, prev_x, number_str)) = number_under_construction.take() {
                    if prev_x + 1 == x {
                        // n-digit number continues
                        number_under_construction = Some((start_x, x, number_str + c));
                    } else {
                        // new n-digit number starts, store previous one
                        found_numbers.push((y, start_x, number_str));
                        number_under_construction = Some((x, x, c.to_string()));
                    }
                } else {
                    number_under_construction = Some((x, x, c.to_string()));
                }
            });
            if let Some((start_x, _, number_str)) = number_under_construction.take() {
                found_numbers.push((y, start_x, number_str));
            }
        });
    println!("{:?}", found_numbers);
    let mut graph = Graph::new_undirected();
    let mut nodes = hashmap! {};
    found_numbers.iter().for_each(|num| {
        build_graph(num, &symbol_mask, &mut graph, &mut nodes);
    });
    let graph = graph;
    println!("The graph is {:?}", graph);
    let sum: usize = graph.node_indices().map(|n| {
        match graph.node_weight(n).unwrap() {
            Node::Gear { .. } => {
                let neighbors = graph.neighbors(n).take(3).collect_vec();
                if neighbors.len() == 2 {
                    let n1 = graph.node_weight(neighbors[0]).unwrap();
                    let n2 = graph.node_weight(neighbors[1]).unwrap();
                    let m = match (n1, n2) {
                        (Node::Num(n1), Node::Num(n2)) => {
                            n1 * n2
                        }
                        _ => { 0 }
                    };
                    println!("{}", m);
                    m
                } else { 0 }
            }
            _ => { 0 }
        }
    }).sum();

    println!("Part II: The sum is {}", sum);
}

fn build_graph(
    number_pos: &(usize, usize, String),
    mask: &BitMatrix,
    graph: &mut Graph<Node, Node, Undirected>,
    nodes: &mut HashMap<Node, NodeIndex>,
) {
    let (y, x_start, s) = number_pos;
    let len = s.len();
    for yy in y.saturating_sub(1)..=(y + 1).min(mask.size().0 - 1) {
        for xx in x_start.saturating_sub(1)..=(x_start + len).min(mask.size().1 - 1) {
            if mask[yy][xx] {
                // we found a new edge!
                let gear_node = Node::Gear { y: yy, x: xx };
                let num_node = Node::Num(usize::from_str(s).unwrap());
                let gear_node = nodes.get(&gear_node).cloned().unwrap_or_else(|| {
                    let node_idx = graph.add_node(gear_node.clone());
                    nodes.insert(gear_node.clone(), node_idx);
                    node_idx
                });
                let num_node = nodes.get(&num_node).cloned().unwrap_or_else(|| {
                    let node_idx = graph.add_node(num_node.clone());
                    nodes.insert(num_node.clone(), node_idx);
                    node_idx
                });
                graph.extend_with_edges(
                    &[
                        (gear_node, num_node)
                    ]
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {}
}