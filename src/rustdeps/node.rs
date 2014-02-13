use path::Path;

#[deriving(Eq, Clone)]
pub struct Node
{
    path: ~Path,
    name: ~str,
}

impl Node
{
    pub fn new(path: &Path) -> Node
    {
        Node {
            path: ~path.clone(),
            name: path.name(),
        }
    }
}
