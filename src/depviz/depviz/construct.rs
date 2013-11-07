use std::path::Path;

use depviz::Node;
use depviz::helper;

pub fn construct_crate(name: ~str, path: Path) -> Node
{
    let mut root = Node::new(name, path.clone());
    let crate = helper::parse_crate(path.clone());

    root
}
