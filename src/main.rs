#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;
use std::fs::File;
use scanner::Scanner;

pub mod parser;
pub mod scanner;

struct Args {
    input: String,
    uf: i8,
    lvn: bool,
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        // init new args struct
        let mut new_args = Args {
            input: String::new(),
            uf: 1,
            lvn: false,
        };
        for i in 1..args.len() {
            if args[i].contains("-uf") {
                new_args.uf = args[i + 1].to_string().parse::<i8>().unwrap_or_else(|_| {
                    println!("Value passed to -uf should be an integer");
                    std::process::exit(1);
                });
            } else if args[i].contains("-c") {
                new_args.lvn = true;
            } else {
                new_args.input = args[i].clone();
            }
        }
        if new_args.input.is_empty() {
            return Err("No input file");
        }
        Ok(new_args)
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = Args::new(&env_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    // let f = File::open(&args.input);
    let f_contents = fs::read_to_string(&args.input).unwrap_or_else(|_| {
        println!("Error opening file");
        std::process::exit(1);
    });

    let mut scanner = Scanner::new(f_contents);

    loop {
        let tok = &scanner.token();
        if tok.is_none() {
            break;
        }
        println!("{:?}", tok);
    }
    println!("lineno: {:?}", scanner.lineno);
}
