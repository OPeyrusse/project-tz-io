#[macro_use]
extern crate nom;

mod parser;

fn main() {
    println!("Hello, world!");
    parser::parse(b"aabba");
}