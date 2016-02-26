extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print the help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        println!("You've asked for help!");
    } else {
        println!("You just want to run the program!");
    }
}
