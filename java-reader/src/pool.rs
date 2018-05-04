use std::io;
use reader::{Reader, to_u16};
use printer::print_bytes;

#[derive(Debug)]
pub enum PoolElement {
  Utf8Value(String)
}

pub type PoolList = Vec<Option<PoolElement>>;

fn read_entry(reader: &mut Reader) -> io::Result<PoolElement> {
  let entry_code = reader.read_1u()?;
  print_bytes(1, entry_code);

  Ok(PoolElement::Utf8Value(
        String::from("entry")))
}

pub fn read_class_pool(reader: &mut Reader) -> io::Result<PoolList> {
	let count: u16;
	{ 
		let bytes = reader.read_2u()?;
		count = to_u16(bytes);
		print_bytes(0, bytes);
		println!("constant pool size = {}", count);
	}

	let mut entries = vec![None];
	for _i in 1..count {
    let entry = read_entry(reader)?;
    entries.push(Some(entry));
	}

	Ok(entries)
}