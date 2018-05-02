use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

use generator::java::class::{
  JavaClass,
  PoolElement
};
use generator::java::constants;
use generator::java::constructs::{
  Attribute,
  Operation
};

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

fn write_u32(file: &mut File, value: u32) -> io::Result<()> {
  let buf: [u8; 4] = [
    (value >> 24) as u8,
    ((value >> 16) & 0xff) as u8,
    ((value >> 8) & 0xff) as u8,
    (value & 0xff) as u8 
  ];
  file.write_all(&buf)
}

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
        write_u8(file, constants::PoolCode::Utf8 as u8)?;
        write_u16(file, value.len() as u16)?;
        write_string(file, value)?;
      },
      &PoolElement::ClassInfo(c_idx) => {
        write_u8(file, constants::PoolCode::Class as u8)?;
        write_u16(file, c_idx)?;
      },
      &PoolElement::Integer(value) => {
        write_u8(file, constants::PoolCode::Integer as u8)?;
        write_u32(file, value)?;
      },
      &PoolElement::MethodRef(class_idx, name_idx) => {
        write_u8(file, constants::PoolCode::MethodRef as u8)?;
        write_u16(file, class_idx)?;
        write_u16(file, name_idx)?;
      },
      &PoolElement::NameAndType(name_idx, descriptor_idx) => {
        write_u8(file, constants::PoolCode::NameAndType as u8)?;
        write_u16(file, name_idx)?;
        write_u16(file, descriptor_idx)?;
      }
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
  write_u16(file, class.interfaces.len() as u16)?;
  for interface_id in &class.interfaces {
    write_u16(file, *interface_id)?;
  }
  file.flush()
}

fn write_class_definition(file: &mut File, class: &JavaClass) -> io::Result<()> {
  // TODO write the correct writer
  // No fields
  write_u16(file, 0)?;

  // Write methods
  write_u16(file, class.methods.len() as u16)?;
  for method in &class.methods {
    write_u16(file, method.access)?;
    write_u16(file, method.name_index)?;
    write_u16(file, method.descriptor_index)?;
    write_u16(file, method.attributes.len() as u16)?;
    for entry in &method.attributes {
      write_attribute(file, entry)?;
    }
  }
  
  // No attributes
  write_u16(file, 0)?;

  file.flush()
}

fn write_attribute(file: &mut File, &(ref idx, ref attribute): &(u16, Attribute)) -> io::Result<()> {
  match attribute {
    &Attribute::Code(ref max_stack, ref operations) => {
      write_u16(file, *idx)?;

      let attribute_length: u32 = 1234; // TODO compute the correct length
      write_u32(file, attribute_length)?;
      write_u16(file, *max_stack)?;

      let local_vars_count: u8 = operations.iter()
        .map(|op| match op {
          &Operation::aload(ref idx) => *idx,
          &Operation::astore(ref idx) => *idx,
          _ => 0u8
        })
        .max().unwrap_or(0u8);
      write_u16(file, local_vars_count as u16)?;
      let code_length: u32 = 234; // TODO: compute the correct code length
      write_u32(file, code_length)?;
      for operation in operations {
        write_operation(file, operation)?;
      }

      // Not used so far
      write_u16(file, 0)?; // No exception tables
      write_u16(file, 0) // No attributes
    }
  }
}

fn write_operation(file: &mut File, operation: &Operation) -> io::Result<()> {
  match operation {
    &Operation::aload(ref idx) => {
      // if idx > 3 { // TODO write the optimization
      write_u8(file, 25)?;
      write_u8(file, *idx)
    },
    &Operation::astore(ref idx) => {
      write_u8(file, 58)?;
      write_u8(file, *idx)
    },
    &Operation::iastore => {
      write_u8(file, 79)
    },
    &Operation::iconst_1 => {
      write_u8(file, 4)
    },
    &Operation::invokespecial(ref idx) => {
      write_u8(file, 183)?;
      write_u16(file, *idx)
    },
    &Operation::invokevirtual(ref idx) => {
      write_u8(file, 182)?;
      write_u16(file, *idx)
    },
    &Operation::ldc(ref idx) => {
      // Optimize using ldc or ldc_w
      if *idx < 256 {
        write_u8(file, 18)?;
        write_u8(file, (*idx & 0xff) as u8)
      } else {
        write_u8(file, 19)?;
        write_u16(file, *idx)
      }
    },
    &Operation::new(ref idx) => {
      write_u8(file, 187)?;
      write_u16(file, *idx)
    },
    &Operation::newarray(ref array_type) => {
      write_u8(file, 188)?;
      write_u8(file, array_type.clone() as u8)
    }
  }
}

pub fn write(class: &JavaClass, output_file: &Path) -> io::Result<()> {
  // println!("class: {:?}", class);
  let mut file = File::create(output_file)?;
  write_header(&mut file)?;
  write_constant_pool(&mut file, class)?;
  write_class_info(&mut file, class)?;
  write_class_definition(&mut file, class)
}