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

fn read_file(filename: &str) -> ReadResult {
	println!("Reading {}", filename);
	let mut f = File::open(filename).expect("file not found");
	let mut reader = Reader::new(f);

	let magic_number = reader.read_4u()?;
	for b in magic_number {
		print!("{} ", b);
	}
	println!("magic number");

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