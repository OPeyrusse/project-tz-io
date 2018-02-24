#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

mod parser;

fn parse_file<'a>(filename: &str) -> parser::ParsingResult<'a> {
	println!("Compiling {}", filename);
	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	parser::parse(contents.as_bytes())
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];
	let result = parse_file(filename);
	match result {
		Result::Ok(res) => println!("{:?}", res),
		_ => println!("Failure")
	}
}

#[cfg(test)]
mod tests {
	use super::parse_file;

	#[test]
	fn test_sample_sum() {
		let _r = parse_file("../language-samples/sum.io");
	}

	#[test]
	fn test_sample_increment() {
		let _r = parse_file("../language-samples/increment.io");
	}

}
