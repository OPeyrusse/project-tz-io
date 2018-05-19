use pool::{PoolList, resolve_utf8_value};
use printer::print_bytes;
use reader::{Reader, ReadResult, to_u16, to_u32};

pub fn read(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  read_u16!(attribute_idx, reader, indent);
  let attribute_name = resolve_utf8_value(pool, attribute_idx as usize)
    .expect(&format!(
      "No attribute name in constant pool at {}",
      attribute_idx));
  println!("Attribute '{}'", attribute_name);

  read_u32!(length, reader, indent + 1);
  println!("Attribute length = {}", length);

  match attribute_name {
    "Code" => read_code(reader, pool, length, indent),
    _ => panic!("Unsupported attribute '{}'", attribute_name)
  }
}

fn read_code(reader: &mut Reader, pool: &PoolList, length: u32, indent: u8) -> ReadResult {
  let mut bytes = vec![0; length as usize];
  reader.read(&mut bytes[..])?;
  print_bytes(indent, &bytes[..]);
  println!("Read: {}", length);
  Ok(())
}