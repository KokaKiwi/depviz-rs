use std::hashmap::HashMap;
use path::Path;
use node::Node;

pub struct RustDeps
{
    nodes: ~HashMap<Path, Node>,
}

impl RustDeps
{
    pub fn new() -> RustDeps
    {
        RustDeps {
            nodes: ~HashMap::new(),
        }
    }

    pub fn add(&mut self, node: Node)
    {
        self.nodes.insert(*(node.path.clone()), node);
    }
}
