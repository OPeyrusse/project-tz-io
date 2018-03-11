#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser;
mod postprocessor;
mod checker;

fn parse_file(filename: &str) -> parser::ParsingResult {
	println!("Compiling {}", filename);
	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	parser::parse(contents.as_bytes())
}

fn process_file(filename: &str) -> parser::ParsingResult {
	let mut result = parse_file(filename)
		.map(|mut r| {
			postprocessor::process(&mut r);
			r
		});
	checker::check(&result);
	result
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];
	let _res = process_file(filename);
}

#[cfg(test)]
mod tests {
	use super::process_file;

	#[test]
	fn test_sample_sum() {
		let res = process_file("../language-samples/sum.io");
		assert_eq!(res.is_ok(), true);
	}

	#[test]
	fn test_sample_increment() {
		let res = process_file("../language-samples/increment.io");
		assert_eq!(res.is_ok(), true);
	}

}
