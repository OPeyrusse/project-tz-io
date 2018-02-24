#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Compiling {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

//     parser::parse(b"
//  Node #1
// =========
// IN:1 -> 1
// -------
// MOV <1, ACC
// SAV
// ADD 1
// SWP
// SUB 1
// MOV ACC, >1
// SWP
// MOV ACC, >2
// --------
// 1 -> OUT:1, 2 -> OUT:2
// ============
// ");
    let result = parser::parse(contents.as_bytes());
    match result {
        Result::Ok(res) => println!("{:?}", res),
        _ => println!("Failure")
    }
}
