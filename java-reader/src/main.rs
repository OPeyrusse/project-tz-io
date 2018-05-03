use std::env;
use std::io;
use std::io::{File, BufReader};

type ReadResult = io::Result<()>;

struct Reader<'a> {
	buffer: BufReader,
	buf1: [u8; 100],
	buf2: [u8; 100]
}

impl <'a> Reader<'a> {
	pub fn new(file: &'a mut File) {
		Reader {
			buffer: BufReader::new(file)
		}
	}
}

fn read_file(filename: &str) -> ReadResult {
	println!("Reading {}", filename);
	let mut f = File::open(filename).expect("file not found");

	let mut reader = Reader::new(f);
	Ok(())
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];
}