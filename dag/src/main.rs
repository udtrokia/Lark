extern crate dag;
use dag::graph::Graphadj;
use dag::node::Node;

fn main() {
    println!("Hello, world!");
    let mut g = Graphadj::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(1, "v2".to_string());
    g.insert_edge(v1, v2);
    println!("{:?}", g);
}
