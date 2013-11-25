#[link(
    name = "depviz",
    vers = "0.1.0",
    uuid = "aecc116d-c1bd-4847-9435-f020d565e181"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "bin"];

#[feature(managed_boxes)];
#[feature(macro_rules)];

extern mod extra;
extern mod syntax;
extern mod argparse;

use std::os;
use std::path::Path;

use argparse::ArgumentParser;
use argparse::arg;

use depviz::Node;
use depviz::construct;

#[path = "../../deps/argparse-rs/src/argparse/macros.rs"]
mod argparse_macros;

mod depviz;

static BRIEF_USAGE: &'static str = "Rust crate dependencies visualization.";

fn main_args(args: &[~str]) -> int
{
    let mut parser = ArgumentParser::new();
    parser.description = Some(BRIEF_USAGE);

    let opts = ~[
        create_arg!("-h", "--help"; ty = arg::ArgTyBool, help = Some("Show this help and exit.")),
        create_arg!("filename"),
    ];
    parser.add_arguments(opts);

    let args = match parser.parse_args(args.tail()) {
        Ok(args) => args,
        Err(e) => {
            println!("Error: {}", e.to_str());
            parser.print_help();
            return 1;
        }
    };

    if args.get::<bool>("help")
    {
        parser.print_help();
        return 0;
    }

    let filename = args.get::<~str>("filename");

    let path = Path::new(filename);
    let name = match path.filestem_str() {
        Some(s) => {
            if s == "lib"
            {
                let dir_path = path.dir_path();
                match dir_path.filestem_str()
                {
                    Some(d) => d.slice_from(3).to_owned(),
                    None => s.to_owned(),
                }
            }
            else
            {
                s.to_owned()
            }
        }
        None => fail!(),
    };

    let mut constructor = construct::NodeConstructor::new();

    let root = constructor.construct_crate(name.clone(), path, name.clone());
    dot_output(root);

    return 0;
}

fn dot_output(root: &Node)
{
    println("digraph rust {");

    dot_trace(root);

    println("}");
}

fn dot_trace(node: &Node)
{
    println!("  {} [label=\"{}\"];", node.ast_path, node.name);
    for child in node.children.iter()
    {
        dot_trace(*child);
        println!("    {} -> {};", node.ast_path, child.ast_path);
    }
}

fn main()
{
    os::set_exit_status(main_args(os::args()));
}
