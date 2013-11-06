use std::path::Path;

pub mod construct;

pub struct Node
{
    parent: Option<@Node>,

    name: ~str,
    path: Path,
}
