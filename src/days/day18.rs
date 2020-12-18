use crate::common::Solution;

#[derive(PartialEq)]
enum Op {
    Add,
    Mul,
}

enum Node {
    Value(i128),
    Expr(Box<Node>, Vec<(Op, Node)>),
    Paren(Box<Node>),
}

fn merge(lhs: Option<Node>, op: Option<Op>, rhs: Node) -> Node {
    use Node::*;
    match (lhs, op) {
        (Some(lhs), Some(op)) => match lhs {
            l @ Value(_) => Expr(Box::new(l), vec![(op, rhs)]),
            l @ Paren(_) => Expr(Box::new(l), vec![(op, rhs)]),
            Expr(first, mut rest) => {
                rest.push((op, rhs));
                Expr(first, rest)
            }
        },
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
            (Node::Paren(Box::new(lhs.unwrap())), &input[1..])
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
    use Node::*;
    match expr {
        Value(v) => *v,
        Paren(e) => eval(e),
        Expr(first, rest) => rest
            .into_iter()
            .fold(eval(&first), |acc, (op, next)| match op {
                Op::Add => acc + eval(next),
                Op::Mul => acc * eval(next),
            }),
    }
}

fn prioritize_add(expr: Node) -> Node {
    use Node::*;
    match expr {
        v @ Value(_) => v,
        Paren(e) => Paren(Box::new(prioritize_add(*e))),
        Expr(lhs, rest) => {
            let new_rest = rest.into_iter().fold(vec![], |mut acc, (op, next)| {
                if op == Op::Add && !acc.is_empty() {
                    let (op_last, last) = acc.pop().unwrap();
                    acc.push((
                        op_last,
                        Expr(Box::new(last), vec![(Op::Add, prioritize_add(next))]),
                    ));
                } else {
                    acc.push((op, prioritize_add(next)));
                }
                acc
            });

            Expr(Box::new(prioritize_add(*lhs)), new_rest)
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let exprs: Vec<Node> = lines.iter().map(|l| parse(None, None, l).0).collect();
    (
        exprs.iter().map(|e| eval(e)).sum::<i128>().to_string(),
        exprs
            .into_iter()
            .map(|e| prioritize_add(e))
            .map(|e| eval(&e))
            .sum::<i128>()
            .to_string(),
    )
}
