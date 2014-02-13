#[crate_id = "rustdeps#0.1.0"];
#[comment = "A Rust dependencies resolver library."];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod syntax;

pub use deps::RustDeps;
pub use node::Node;

use std::os;

pub mod node;
pub mod path;
pub mod deps;
pub mod helper;

pub fn parse_file(_path: &Path) -> RustDeps
{
    let root = RustDeps::new();

    root
}
