use crate::common::Solution;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Node {
    Value(i128),
    Expr(Box<Node>, Op, Box<Node>),
}

fn merge(lhs: Option<Node>, op: Option<Op>, rhs: Node) -> Node {
    match (lhs, op) {
        (Some(lhs), Some(op)) => Node::Expr(Box::new(lhs), op, Box::new(rhs)),
        (None, None) => rhs,
        _ => panic!("Invalid state"),
    }
}

fn parse(lhs: Option<Node>, op: Option<Op>, mut input: &str) -> (Node, &str) {
    input = input.trim();

    match input.chars().next() {
        None => {
            assert!(op.is_none());
            (lhs.unwrap(), input)
        }

        Some('(') => {
            let (rhs, remaining) = parse(None, None, &input[1..]);
            parse(Some(merge(lhs, op, rhs)), None, remaining)
        }

        Some(')') => {
            assert!(op.is_none());
            (lhs.unwrap(), &input[1..])
        }

        Some('+') => parse(lhs, Some(Op::Add), &input[1..]),

        Some('*') => parse(lhs, Some(Op::Mul), &input[1..]),

        _ => {
            let (rhs, remaining) =
                if let Some(split_i) = input.find(|c: char| !('0'..='9').contains(&c)) {
                    (
                        Node::Value(input[..split_i].trim().parse().unwrap()),
                        &input[split_i..],
                    )
                } else {
                    (Node::Value(input.parse().unwrap()), "")
                };

            parse(Some(merge(lhs, op, rhs)), None, remaining)
        }
    }
}

fn eval(expr: &Node) -> i128 {
    match expr {
        Node::Value(v) => *v,
        Node::Expr(a, op, b) => match op {
            Op::Add => eval(a) + eval(b),
            Op::Mul => eval(a) * eval(b),
        },
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let exprs: Vec<Node> = lines.iter().map(|l| parse(None, None, l).0).collect();
    (
        exprs.iter().map(|e| eval(e)).sum::<i128>().to_string(),
        "".to_string(),
    )
}
