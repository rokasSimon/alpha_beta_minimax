#![allow(arithmetic_overflow)]
#![allow(non_snake_case)]

use colored::Colorize;

enum Node<'a> {
    Leaf(&'a str, i32),
    Node(&'a str, Vec<Node<'a>>)
}

fn log_skipped_nodes<'a>(nodes: &[Node<'a>]) {
    for node in nodes {
        match node {
            Node::Leaf(name, leaf_value) => {
                println!("{}", format!(
                    "Skipped leaf '{}' with value {}", name, leaf_value
                ).red());
            },
            Node::Node(name, children) => {
                println!("{}", format!(
                    "Skipped node '{}'", name
                ).red());

                log_skipped_nodes(children);
            }
        };
    }
}

fn alpha_beta(node: &Node, depth: i32, mut alpha: i32, mut beta: i32, is_maximising_player: bool) -> i32 {
    match node {
        Node::Leaf(name, leaf_value) => {
            println!("Visited: '{}'", name.green());

            *leaf_value
        },
        Node::Node(name, children) => {
            println!("Visited: '{}'", name.green());

            if is_maximising_player {
                let mut best_val = i32::MIN;

                for (i, child) in children.iter().enumerate() {
                    let value = alpha_beta(child, depth + 1, alpha, beta, false);
                    best_val = i32::max(best_val, value);
                    alpha = i32::max(alpha, best_val);

                    if alpha >= beta {
                        log_skipped_nodes(&children[i+1..]);
                        break;
                    }
                }
                best_val

            } else {
                let mut best_val = i32::MAX;

                for (i, child) in children.iter().enumerate() {
                    let value = alpha_beta(child, depth + 1, alpha, beta, true);
                    best_val = i32::min(best_val, value);
                    beta = i32::min(beta, best_val);

                    if alpha >= beta {
                        log_skipped_nodes(&children[i+1..]);
                        break;
                    }
                }
                best_val
            }
        }
    }
}

fn main() {
    /*let tree = Node::Node("a", vec![
       Node::Node("b", vec![
           Node::Node("e", vec![
               Node::Leaf("0", 0),
               Node::Leaf("7", 7)
           ]),
           Node::Node("f", vec![
               Node::Leaf("15", 15)
           ])
       ]),
       Node::Node("c", vec![
           Node::Node("g", vec![
               Node::Leaf("7", 7),
               Node::Leaf("3", 3)
           ]),
           Node::Node("h" , vec![
               Node::Leaf("6", 6),
               Node::Leaf("8", 8)
           ])
       ]),
       Node::Node("d", vec![
           Node::Node("i", vec![
               Node::Leaf("6", 6),
               Node::Leaf("5", 5)
           ]),
           Node::Node("j" , vec![
               Node::Leaf("0", 0),
               Node::Leaf("1", 1)
           ])
       ])
    ]);*/

    let tree = Node::Node("a", vec![
        Node::Node("b", vec![
            Node::Node("d", vec![
                Node::Leaf("i", -2),
                Node::Leaf("j", 5)
            ]),
            Node::Node("e", vec![
                Node::Leaf("k", 9)
            ]),
            Node::Node("f", vec![
                Node::Leaf("m", 5),
                Node::Leaf("n", 6)
            ]),
        ]),
        Node::Node("c", vec![
            Node::Node("g", vec![
                Node::Leaf("l", -3),
                Node::Leaf("o", -4)
            ]),
            Node::Node("h" , vec![
                Node::Leaf("p", 8),
                Node::Leaf("r", 0)
            ])
        ])
     ]);

    let output = alpha_beta(&tree, 0, i32::MIN, i32::MAX, true);

    println!("{}", format!("Output value is {}", output).blue());
}
