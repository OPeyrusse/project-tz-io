#[macro_use]
extern crate nom;

mod parser;

fn main() {
    println!("Hello, world!");
    parser::parse(b"
 Node #1
=========
IN:1 -> 1
-------
MOV <1, ACC
SAV
ADD 1
SWP
SUB 1
MOV ACC, >1
SWP
MOV ACC, >2
--------
1 -> OUT:1, 2 -> OUT:2
============
");
}
