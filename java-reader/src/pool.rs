use std::io;
use reader::{Reader, to_u16};
use printer::print_bytes;
use std::str::from_utf8;

#[derive(Debug)]
pub enum PoolElement {
  Utf8Value(String)
}

pub type PoolList = Vec<Option<PoolElement>>;

fn read_entry(reader: &mut Reader) -> io::Result<PoolElement> {
  let pool_code: u8;
  {
    let entry_code = reader.read_1u()?; 
    print_bytes(1, entry_code);
    pool_code = entry_code[0];
  }

  let element = match pool_code {
    1 => {
      println!("Utf8 constant");

      let length: u16;
      { 
        let length_bytes = reader.read_2u()?;
        print_bytes(2, length_bytes);
        length = to_u16(length_bytes);
        println!("length {}", length);
      }

      let value: String;
      { 
        let content = reader.read_up_to_u16(length)?; 
        print_bytes(2, content);
        value = String::from(from_utf8(content).expect("Invalid utf8 content"));
      }
      // TODO support the full string encoding
      PoolElement::Utf8Value(value)
    },
    _ => panic!("Unsupported pool element. Code = {}", pool_code)
  };
  println!("{:?}", element);
  Ok(element)
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