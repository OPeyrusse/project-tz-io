use std::io;
use std::io::{BufReader, Read};
use std::fs::File;

pub type ReadResult = io::Result<()>;

pub struct Reader {
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

  pub fn read_up_to_u16(&mut self, length: u16) -> io::Result<&[u8]> {
    if length <= 100 {
      let end = length as usize;
      self.buffer.read_exact(&mut self.data_buffer[0..end])?;
      Ok(&self.data_buffer[0..end])
    } else {
      panic!("Not supporting read > 100 chars yet. Asked: {}", length);
    }
  }
}

pub fn to_u16(bytes: &[u8]) -> u16 {
	((bytes[0] as u16) << 8) | (bytes[1] as u16)
}

pub fn to_u32(bytes: &[u8]) -> u32 {
	((bytes[0] as u32) << 24) 
	  | ((bytes[1] as u32) << 16) 
	  | ((bytes[2] as u32) << 8) 
    | (bytes[3] as u32)
}