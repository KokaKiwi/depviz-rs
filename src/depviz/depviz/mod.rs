use std::path::Path;

pub mod construct;
pub mod helper;

pub struct Node
{
    name: ~str,
    path: Option<Path>,

    children: ~[Node],
}

impl Node
{
    pub fn new(name: ~str, path: Path) -> Node
    {
        Node {
            name: name,
            path: Some(path),

            children: ~[],
        }
    }

    pub fn new_extern(name: ~str) -> Node
    {
        Node {
            name: name,
            path: None,

            children: ~[],
        }
    }
}
