mod generator;

use std::io::{File, BufReader};

struct Reader<'a> {
	buffer: BufReader
}

impl <'a> Reader<'a> {
	pub fn new(file: &'a mut File) {
		Reader {
			buffer: BufReader::new(file)
		}
	}
}

fn read_file(filename: &str) -> ParsingResult {
	println!("Reading {}", filename);
	let mut f = File::open(filename).expect("file not found");

	let mut reader = Reader::new(f);
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];
}