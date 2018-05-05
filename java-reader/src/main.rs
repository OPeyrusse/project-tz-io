mod reader;
mod pool;
mod printer;

use std::env;
use std::fs::File;
use printer::print_bytes;

use reader::{Reader, ReadResult, to_u16};

fn read_header(reader: &mut Reader) -> ReadResult {
	{
		let magic_number = reader.read_4u()?;
		print_bytes(0, magic_number);
		println!("magic number");
	}

	{
		let versions = reader.read_4u()?;
		print_bytes(0, versions);
		let minor = to_u16(&versions[0..2]);
		let major = to_u16(&versions[2..4]);
		println!("version: {}.{}", major, minor);
	}

	Ok(())
}

fn read_file(filename: &str) -> ReadResult {
	println!("Reading {}", filename);
	let f = File::open(filename).expect("file not found");
	let mut reader = Reader::new(f);

	read_header(&mut reader)?;
	let _pool = pool::read_class_pool(&mut reader)?;

	Ok(())
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	match read_file(filename) {
		Ok(_) => println!("File read without error"),
		Err(e) => panic!("Read failed with error {}", e)
	}
}