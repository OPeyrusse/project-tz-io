use pool::{PoolList, resolve_method_name};
use printer::{print_bytes};
use reader::{Reader, ByteReader, ReadResult, to_u16};
use types::{ARRAY_TYPES};

fn print_op(name: &str) {
  println!("[{}]", name);
}

pub fn read(
    reader: &mut ByteReader, 
    pool: &PoolList,
    indent: u8) -> ReadResult {
  while !reader.is_empty() {
    read_u8!(operation_code, reader, indent);
    match operation_code {
      2 ... 8 => read_iconst(operation_code),
      18 => read_ldc(reader, indent)?,
      25 => read_aload(reader, indent)?,
      58 => read_astore(reader, indent)?,
      79 => read_iastore(),
      182 => read_invoke_virtual(reader, pool, indent)?,
      188 => read_new_array(reader, indent)?,
      _ => panic!("Unsupported operation: {}", operation_code)
    }
  }
  Ok(())
}

fn read_aload(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("aload");
  read_u8!(var_idx, reader, indent + 1);
  println!("load from var#{}", var_idx);

  Ok(())
}

fn read_astore(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("astore");
  read_u8!(var_idx, reader, indent + 1);
  println!("strore into var#{}", var_idx);

  Ok(())
}

fn read_iastore() {
  print_op("iastore");
}

fn read_iconst(operation: u8) {
  let num = operation as i8 - 3;
  match num {
    -1 => print_op("iconst_m1"),
    0 ... 5 => print_op(&format!("iconst_{}", num)),
    _ => panic!("Invalid constant value {}", num)
  }
}

fn read_invoke_virtual(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("invokevirtual");
  read_u16!(method_idx, reader, indent);
  let (ref method_name, ref descriptor) = resolve_method_name(pool, method_idx as usize)
    .expect(&format!("No method reference in pool at {}", method_idx));
  println!("Invoke #{} = {}{}", method_idx, method_name, descriptor);
  Ok(())
}

fn read_new_array(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("newarray");
  read_u8!(array_type, reader, indent + 1);
  let type_name = ARRAY_TYPES.get(&array_type)
    .expect(&format!("No array type with code {}", array_type));
  println!("new array of {}", type_name);

  Ok(())
}

fn read_ldc(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("ldc");
  read_u8!(idx, reader, indent + 1);
  println!("load constant #{}", idx);
  Ok(())
}

