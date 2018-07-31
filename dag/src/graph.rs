// graph.rs
use edge::Edge;
use node::Node;

#[derive(Debug)]
pub struct Graphadj {
    nodenums: usize,
    graphadj: Vec<Vec<Edge>>,
}

impl Graphadj {
    pub fn new(nums: usize) -> Graphadj {
        Graphadj {
            nodenums: nums,
            graphadj: vec![vec![Edge::new(); nums]; nums],
        }
    }
    
    pub fn insert_edge(&mut self, v1: Node, v2: Node) {
        match v1.nodeid < self.nodenums {
            true => {
                self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
                // self.graphadj[v2.nodeid][v1.nodeid] = Edge::have_edge();
            }
            false => {
                panic!("your nodeid is bigger than nodenums!");
            }
        }
    }
}
