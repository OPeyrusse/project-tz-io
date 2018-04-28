use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

use generator::java::class::{
  JavaClass,
  PoolElement
};
use generator::java::constants;

static MAGIC: [u8; 4] = [0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8];
static VERSIONS: [u8; 4] = [/* minor */0, 0, /* major */0, 52];

// static DEBUG: bool = true;
// macro_rules! debug {
//   ($fmt:expr) => {
//     if DEBUG {
//       println!($fmt);
//     }
//    };
//   ($fmt:expr, $($arg:tt)*) => {
//     if DEBUG {
//       println!($fmt, $($arg)*);
//     }
//    };
// }

fn write_u8(file: &mut File, value: u8) -> io::Result<()> {
  let buf: [u8; 1] = [value];
  file.write_all(&buf)
}

fn write_u16(file: &mut File, value: u16) -> io::Result<()> {
  let buf: [u8; 2] = [
    (value >> 8) as u8,
    (value & 0xff) as u8 
  ];
  file.write_all(&buf)
}

// fn write_u32(file: &mut File, value: u32) -> io::Result<()> {
//   let buf: [u8; 4];
//   unsafe {
//     buf = transmute::<u32, [u8; 4]>(value);
//   }
//   file.write_all(&buf)
// }

fn write_string(file: &mut File, value: &String) -> io::Result<()> {
  let bytes = value.as_bytes();
  if !bytes.iter().all(|c| 0 < *c && *c < 128) {
    panic!("Unsupported chars in the string: `{}`", value);
  } 
  file.write_all(bytes)
}

fn write_header(file: &mut File) -> io::Result<()> {
  file.write_all(&MAGIC)?;
  file.write_all(&VERSIONS)?;
  file.flush()
}

fn write_constant_pool(file: &mut File, class: &JavaClass) -> io::Result<()> {
  let pool_size = class.pool_size();
  write_u16(file, pool_size)?;

  for (_idx, element) in class.pool_iter() {
    match element {
      &PoolElement::Utf8Value(ref value) => {
        write_u8(file, (constants::PoolCode::Utf8 as u8))?;
        write_u16(file, (value.len() as u16))?;
        write_string(file, value)?;
      },
      &PoolElement::ClassInfo(c_idx) => {
        write_u8(file, (constants::PoolCode::Class as u8))?;
        write_u16(file, c_idx)?;
      },
      _ => ()
    } 
  }
  file.flush()
}

fn write_class_info(file: &mut File, class: &JavaClass) -> io::Result<()> {
  let access: u16 = (constants::ClassAccess::FINAL as u16)
    | (constants::ClassAccess::SUPER as u16);
  write_u16(file, access)?;
  write_u16(file, class.class_id)?;
  write_u16(file, class.super_class_id)?;

  // For now, tell that there are no interfaces
  write_u16(file, 0)?;
  // TODO print the interfaces
  // write_u8(file, &(class.interfaces.len() as u8))?;
  // for interface_id in &class.interfaces {
  //   write_u16(file, interface_id)?;
  // }
  file.flush()
}

fn write_class_definition(file: &mut File, class: &JavaClass) -> io::Result<()> {
  // TODO write the correct writer
  // No fields
  write_u16(file, 0)?;
  // No methods
  write_u16(file, 0)?;
  // No attributes
  write_u16(file, 0)?;

  file.flush()
}

pub fn write(class: &JavaClass, output_file: &Path) -> io::Result<()> {
  // println!("class: {:?}", class);
  let mut file = File::create(output_file)?;
  write_header(&mut file)?;
  write_constant_pool(&mut file, class)?;
  write_class_info(&mut file, class)?;
  write_class_definition(&mut file, class)
}