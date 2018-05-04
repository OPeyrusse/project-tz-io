mod reader;

use std::env;
use std::io;
use std::fs::File;

use reader::{Reader, ReadResult, to_u16};

fn print_bytes(indent: u8, bytes: &[u8]) {
	for _i in 0..indent {
		print!("  ");
	}
	for b in bytes {
		match *b {
			// Small fix as it is not possible to put trailing 0s in front of hexa
			b @ 0 ... 9 => print!("0{:X} ", b),
			_ => print!("{:X} ", b)
		}
	}
	print!("> ")
}

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

fn read_class_pool(reader: &mut Reader) -> io::Result<()> {
	let count: u16;
	{ 
		let bytes = reader.read_2u()?;
		count = to_u16(bytes);
		print_bytes(0, bytes);
		println!("constant pool size = {}", count);
	}

	Ok(())
}

fn read_file(filename: &str) -> ReadResult {
	println!("Reading {}", filename);
	let f = File::open(filename).expect("file not found");
	let mut reader = Reader::new(f);

	read_header(&mut reader)?;
	read_class_pool(&mut reader)?;

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