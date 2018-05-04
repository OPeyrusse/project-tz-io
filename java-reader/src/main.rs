use std::env;
use std::io;
use std::io::{BufReader, Read};
use std::fs::File;

type ReadResult = io::Result<()>;

struct Reader {
	buffer: BufReader<File>,
	data_buffer: [u8; 100]
}

impl Reader {
	pub fn new(file: File) -> Reader {
		Reader {
			buffer: BufReader::new(file),
			data_buffer: [0; 100]
		}
	}

	pub fn read_1u(&mut self) -> io::Result<&[u8]> {
		self.buffer.read_exact(&mut self.data_buffer[0..1])?;
		Ok(&self.data_buffer[0..1])
	}

	pub fn read_2u(&mut self) -> io::Result<&[u8]> {
		self.buffer.read_exact(&mut self.data_buffer[0..2])?;
		Ok(&self.data_buffer[0..2])
	}

	pub fn read_4u(&mut self) -> io::Result<&[u8]> {
		self.buffer.read_exact(&mut self.data_buffer[0..4])?;
		Ok(&self.data_buffer[0..4])
	}

	// fn read(&mut self, buffer: &mut [u8]) -> ReadResult {
	// 	self.buffer.read_exact(&mut self.data_buffer[0..1])
	// }
}

fn to_u16(bytes: &[u8]) -> u16 {
	((bytes[0] as u16) << 8) | (bytes[1] as u16)
}

fn print_bytes(indent: u8, bytes: &[u8]) {
	for _i in 0..indent {
		print!("  ");
	}
	for b in bytes {
		match *b {
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

fn read_file(filename: &str) -> ReadResult {
	println!("Reading {}", filename);
	let f = File::open(filename).expect("file not found");
	let mut reader = Reader::new(f);

	read_header(&mut reader)?;

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