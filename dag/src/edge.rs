#[derive(Debug, Clone)]
pub struct Edge {
    edge: bool,
}

impl Edge {
    pub fn new() -> Edge { Edge { edge: false }}
    pub fn have_edge() -> Edge { Edge { edge: true }}
}
