extern crate getopts;

use getopts::Options;
use std::env;

mod cache;

const C1_DEFAULT: u8    = 12;
const B1_DEFAULT: u8    = 5;
const S1_DEFAULT: u8    = 3;
const V_DEFAULT:  u8    = 3;
const C2_DEFAULT: u8    = 15;
const B2_DEFAULT: u8    = 5;
const S2_DEFAULT: u8    = 4;

fn print_usage() {
    println!("cachesim [OPTIONS] < traces/file.trace");
    println!("L1 Parameters:");
    println!("  -c C1\t\tThe total size of the L1 cache is 2^C1 bytes");
    println!("  -b B1\t\tThe size of a block in the L1 cache is 2^B1 bytes");
    println!("  -s S1\t\tThe number of blocks per set in L1 is 2^S1");
    println!("Victim cache parameters:");
    println!("  -v V\t\tThe number of blocks in the Victim Cache");
    println!("L2 Parameters:");
    println!("  -C C2\t\tThe total size of the L2 cache is 2^C2 bytes");
    println!("  -B B2\t\tThe size of a block in the L2 cache is 2^B2 bytes");
    println!("  -S S2\t\tThe number of blocks per set in L2 is 2^S2");
}

fn main() {
    let args : Vec<String> = env::args().collect();
    //let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print the help menu");

    opts.optopt("c", "", "C parameter for L1 cache", "C1");
    opts.optopt("b", "", "B parameter for L1 cache", "B1");
    opts.optopt("s", "", "S parameter for L1 cache", "S1");
    opts.optopt("v", "", "v parameter for Victim cache", "V");
    opts.optopt("C", "", "C parameter for L2 cache", "C2");
    opts.optopt("B", "", "B parameter for L2 cache", "B2");
    opts.optopt("S", "", "S parameter for L2 cache", "S2");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) },
    };

    if matches.opt_present("h") {
        print_usage();
        return;
    }

    let c1 = match matches.opt_str("c") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => C1_DEFAULT,
    };
    let b1 = match matches.opt_str("b") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => B1_DEFAULT,
    };
    let s1 = match matches.opt_str("s") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => S1_DEFAULT,
    };
    let v = match matches.opt_str("v") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => V_DEFAULT,
    };
    let c2 = match matches.opt_str("C") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => C2_DEFAULT,
    };
    let b2 = match matches.opt_str("B") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => B2_DEFAULT,
    };
    let s2 = match matches.opt_str("S") {
        Some(s) => s.parse::<u8>().unwrap(),
        None => S2_DEFAULT,
    };

    println!("Cache Settings");
    println!("c: {}", c1);
    println!("b: {}", b1);
    println!("s: {}", s1);
    println!("v: {}", v);
    println!("C: {}", c2);
    println!("B: {}", b2);
    println!("S: {}", s2);

}
