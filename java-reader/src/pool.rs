use std::io;
use reader::{Reader, to_u16, to_u32};
use printer::print_bytes;
use std::str::from_utf8;

#[derive(Debug)]
pub enum PoolElement {
  Utf8Value(String),
  ClassInfo(usize),
  Integer(u32),
  NameAndType(usize, usize),
  MethodRef(usize, usize)
}

pub type PoolList = Vec<Option<PoolElement>>;

fn read_utf8_value(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_u16!(length, reader, indent);
  println!("length {}", length);

  let value: String;
  {
    let content = reader.read_up_to_u16(length)?;
    print_bytes(indent, content);
    value = String::from(from_utf8(content).expect("Invalid utf8 content"));
  }
  // TODO support the full string encoding
  Ok(PoolElement::Utf8Value(value))
}

fn read_class_info(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_u16!(idx, reader, indent);
  Ok(PoolElement::ClassInfo(idx as usize))
}

fn read_integer(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_u32!(value, reader, indent);
  Ok(PoolElement::Integer(value))
}

fn read_name_and_type(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  let bytes = reader.read_4u()?;
  print_bytes(indent, bytes);
  let name_idx = to_u16(&bytes[0..2]) as usize;
  let descriptor_idx = to_u16(&bytes[2..4]) as usize;

  Ok(PoolElement::NameAndType(name_idx, descriptor_idx))
}

fn read_method_ref(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  let bytes = reader.read_4u()?;
  print_bytes(indent, bytes);
  let class_idx = to_u16(&bytes[0..2]) as usize;
  let name_and_type_idx = to_u16(&bytes[2..4]) as usize;

  Ok(PoolElement::MethodRef(class_idx, name_and_type_idx))
}

fn read_entry(reader: &mut Reader, index: u16) -> io::Result<PoolElement> {
  let pool_code: u8;
  {
    let entry_code = reader.read_1u()?;
    print_bytes(1, entry_code);
    pool_code = entry_code[0];
  }

  print!("#{} ", index);
  let element = match pool_code {
    1 => {
      println!("Utf8 constant");
      read_utf8_value(reader, 2)?
    },
    3 => {
      println!("Integer constant");
      read_integer(reader, 2)?
    },
    7 => {
      println!("Class info");
      read_class_info(reader, 2)?
    },
    10 => {
      println!("Method ref");
      read_method_ref(reader, 2)?
    },
    12 => {
      println!("Name and type");
      read_name_and_type(reader, 2)?
    }
    _ => panic!("Unsupported pool element. Code = {}", pool_code)
  };
  println!("{:?}", element);
  Ok(element)
}

pub fn read_class_pool(reader: &mut Reader) -> io::Result<PoolList> {
  read_u16!(count, reader, 0);
  println!("constant pool size = {}", count);

	let mut entries = vec![None];
	for i in 1..count {
    let entry = read_entry(reader, i)?;
    entries.push(Some(entry));
	}

	Ok(entries)
}

pub fn resolve_utf8_value<'a>(pool: &'a PoolList, index: usize) -> Option<&'a str> {
	if let &Some(ref entry) = &pool[index] {
		match entry {
      &PoolElement::Utf8Value(ref value) => Some(value),
      &PoolElement::ClassInfo(idx) => {
        let class_entry = &pool[idx];
        if let &Some(PoolElement::Utf8Value(ref value)) = class_entry {
          Some(value)
        } else {
          panic!("Invalid index into pool: {:?}", class_entry);
        }
      },
      _ => None
    }
	} else {
		None
	}
}