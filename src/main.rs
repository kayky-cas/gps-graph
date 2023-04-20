use std::collections::HashSet;

struct Link {
    pos: Pos,
    distance: f32,
}

#[derive(Clone, PartialEq)]
struct Pos(f32, f32);

struct Node {
    pos: Pos,
    neighs: Vec<Link>,
}

struct NavigationMap {
    childs: HashSet<Pos, Node>,
}

fn main() {
    println!("Hello, world!");
}
