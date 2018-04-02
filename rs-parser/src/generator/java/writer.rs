use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

use generator::java::class::JavaClass;

static MAGIC: [u8; 4] = [0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8];
static VERSIONS: [u8; 4] = [/* minor */0, 0, /* major */0, 52];

// fn write_or_panic(file: &mut Write, buf: &[u8]) {
//   if let Err(e) = file.write_all(buf) {
//     panic!("Failed to write into file. Caused by {}", e);
//   }
// }

fn write_header(file: &mut File) -> io::Result<()> {
  // Write the magic number
  // write_or_panic(file, &MAGIC);
  file.write_all(&MAGIC)?;
  // Write class version
  // write_or_panic(file, &VERSIONS);
  file.write_all(&VERSIONS)
}

pub fn write(class: &JavaClass, output_file: &Path) -> io::Result<()> {
  let mut file = File::open(output_file)?;
  write_header(&mut file)
}