use std::path::Path;

pub mod construct;
pub mod helper;

pub struct Node
{
    parent: Option<@Node>,
    children: ~[~Node],

    name: ~str,
    path: Path,
}

impl Node
{
    pub fn new(name: ~str, path: Path) -> Node
    {
        Node {
            parent: None,
            children: ~[],

            name: name,
            path: path,
        }
    }
}
