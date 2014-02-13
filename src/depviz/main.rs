#[crate_id = "depviz#0.1.0"];
#[comment = "A Rust dependencies visualizer."];
#[license = "MIT"];
#[crate_type = "bin"];

extern mod rustdeps;
extern mod getopts;

use getopts::{optflag,getopts,OptGroup,short_usage,usage};
use std::os;

fn print_usage(program: &str, opts: &[OptGroup])
{
    println!("Usage: {} <file>\n", short_usage(program, opts));
    println!("{}", usage("A Rust dependencies visualizer.", opts));
}

fn main()
{
    let args = os::args();
    let program = args[0].as_slice();

    let opts = ~[
        optflag("h", "help", "print this help and exit."),
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };

    if matches.opt_present("h")
    {
        print_usage(program, opts);
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].as_slice()
    } else {
        print_usage(program, opts);
        return;
    };

    let path = match Path::new_opt(input) {
        Some(p) => p,
        None => {
            fail!("Bad input file: {}", input);
        },
    };

    if !path.is_file()
    {
        fail!("Bad input file: {}", input);
    }

    let root = rustdeps::parse_file(&path);
    dot_export(&root);
}

fn dot_export(root: &rustdeps::RustDeps)
{
    println!("digraph rust \\{");

    for node in root.nodes.values()
    {
        dot_output(root, node);
    }

    println!("\\}");
}

fn dot_output(root: &rustdeps::RustDeps, node: &rustdeps::Node)
{
    println!("  {}", node.name);
}
