use std::path::Path;

pub mod construct;
pub mod helper;
pub mod visit;

pub struct Node
{
    name: ~str,
    path: Option<Path>,

    ast_path: ~str,

    children: ~[~Node],
}

impl Node
{
    pub fn new(name: ~str, path: Path, ast_path: ~str) -> ~Node
    {
        ~Node {
            name: name,
            path: Some(path),

            ast_path: ast_path,

            children: ~[],
        }
    }

    pub fn new_extern(name: ~str, ast_path: ~str) -> ~Node
    {
        ~Node {
            name: name,
            path: None,

            ast_path: ast_path,

            children: ~[],
        }
    }
}
