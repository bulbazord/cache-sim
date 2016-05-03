extern crate getopts;
extern crate regex;

use std::env;
use std::io::{self, Read};
use getopts::Options;
use cache::{AccessType, CacheSystem};

mod cache;

const C1_DEFAULT: u64    = 12;
const B1_DEFAULT: u64    = 5;
const S1_DEFAULT: u64    = 3;
const V_DEFAULT:  u64    = 3;
const C2_DEFAULT: u64   = 15;
const B2_DEFAULT: u64   = 5;
const S2_DEFAULT: u64   = 4;

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
    // 1.) Begin by handling parameters
    let args : Vec<String> = env::args().collect();

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

    // parse: String -> Option<u64>
    let parse = |s: String| {
        let result = s.parse::<u64>();
        match result {
            Ok(num) => { Some(num) },
            Err(e) => { panic!(e.to_string()) },
        }
    };

    // The parsing. Maybe I can do some kind of map() instead?
    let c1 = matches.opt_str("c").and_then(|s| parse(s)).unwrap_or(C1_DEFAULT);
    let b1 = matches.opt_str("b").and_then(|s| parse(s)).unwrap_or(B1_DEFAULT);
    let s1 = matches.opt_str("s").and_then(|s| parse(s)).unwrap_or(S1_DEFAULT);
    let v =  matches.opt_str("v").and_then(|s| parse(s)).unwrap_or(V_DEFAULT);
    let c2 = matches.opt_str("C").and_then(|s| parse(s)).unwrap_or(C2_DEFAULT);
    let b2 = matches.opt_str("B").and_then(|s| parse(s)).unwrap_or(B2_DEFAULT);
    let s2 = matches.opt_str("S").and_then(|s| parse(s)).unwrap_or(S2_DEFAULT);

    println!("Cache Settings");
    println!("c: {}", c1);
    println!("b: {}", b1);
    println!("s: {}", s1);
    println!("v: {}", v);
    println!("C: {}", c2);
    println!("B: {}", b2);
    println!("S: {}", s2);

    // 2.) Read the trace file
    // The contents of the trace file should be in trace_buffer
    // if the file was piped in through stdin

    let mut trace_buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut trace_buffer) {
        panic!(e.to_string());
    }

    // 3.) Create cache system and begin processing
    let mut cache_system = CacheSystem::new(c1, b1, s1, v, c2, b2, s2);

    let access_list = trace_buffer.split('\n');

    for access in access_list {
        let tokens: Vec<&str> = access.split_whitespace().collect();
        if tokens.len() != 0 {
            let mode: AccessType = match tokens[0] {
                "r" => AccessType::Read,
                "w" => AccessType::Write,
                 _  => { panic!("Malformed trace file!") },
            };

            let address = match u64::from_str_radix(&tokens[1][2..], 16) {
                Ok(v)  => { v },
                Err(f) => { panic!(f.to_string()) },
            };

            cache_system.cache_access(mode, address);
        }
    }

    // 4.) Finish processing, and print out statistics
    cache_system.complete_cache();

    cache_system.print_statistics();
}
