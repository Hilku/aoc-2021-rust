use eval::{to_value, Expr};

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut object1: Vec<eval::Value> = Vec::new();
    while let Some(line) = lines.next() {
        println!("{:?}", Expr::new(line).exec().unwrap());
    }
}
