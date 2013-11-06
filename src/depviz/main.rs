#[link(
    name = "depviz",
    vers = "0.1.0",
    uuid = "aecc116d-c1bd-4847-9435-f020d565e181"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

extern mod extra;
extern mod syntax;

use std::os;
use extra::getopts::groups;

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

    return 0;
}

fn main()
{
    os::set_exit_status(main_args(os::args()));
}
