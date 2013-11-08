#[link(
    name = "depviz",
    vers = "0.1.0",
    uuid = "aecc116d-c1bd-4847-9435-f020d565e181"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "bin"];

#[feature(managed_boxes)];

extern mod extra;
extern mod syntax;

use std::os;
use std::path::Path;

use extra::getopts::groups;

use depviz::Node;

mod depviz;

static BRIEF_USAGE: &'static str = "Rust crate dependencies visualization.";

fn print_usage(program: &str, opts: &[groups::OptGroup])
{
    println!("Usage: {:s} [OPTIONS] FILE", program);
    println!("");
    print(groups::usage(BRIEF_USAGE, opts));
}

fn main_args(args: &[~str]) -> int
{
    let program = args[0].as_slice();

    let opts = ~[
        groups::optflag("h", "help", "Show this help and exit."),
    ];

    let matches = match groups::getopts(args, opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };

    if matches.opts_present([~"h", ~"help"])
    {
        print_usage(program, opts);
        return 0;
    }

    if matches.free.len() <= 1
    {
        print_usage(program, opts);
        return 1;
    }

    let filename = matches.free[1];
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

    let root = depviz::construct::construct_crate(name, path);
    dot_output(root);

    return 0;
}

// fn print_tree(node: &Node, indent: &str)
// {
//     match node.path
//     {
//         Some(ref path) => {
//             // print(indent);
//             // println!("\\#[path = \"{}\"]", path.display());

//             print(indent);
//             if node.children.len() > 0
//             {
//                 println!("mod {} \\{", node.name);
//             }
//             else
//             {
//                 println!("mod {};", node.name);
//             }

//             for child in node.children.iter()
//             {
//                 print_tree(*child, indent + "  ");
//             }

//             if node.children.len() > 0
//             {
//                 print(indent);
//                 println("}");
//             }
//         }
//         None => {
//             print(indent);
//             println!("extern mod {};", node.name);
//         }
//     }
// }

fn dot_output(root: &Node)
{
    println("digraph rust {");

    dot_trace(root);

    println("}");
}

fn dot_trace(node: &Node)
{
    for child in node.children.iter()
    {
        println!("  {} -> {};", node.name, child.name);
        dot_trace(*child);
    }
}

fn main()
{
    os::set_exit_status(main_args(os::args()));
}
