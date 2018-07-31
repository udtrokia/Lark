// node.rs

#[derive(Debug)]
pub struct Node {
    pub nodeid: usize,
    nodename: String,
}

impl Node {
    pub fn new(nodeid: usize, nodename: String) -> Node {
        Node {
            nodeid: nodeid,
            nodename: nodename,
        }
    }
}
