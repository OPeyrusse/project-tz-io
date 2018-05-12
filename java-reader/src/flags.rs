use reader::{Reader, ReadResult, to_u16};
use printer::print_bytes;

lazy_static! {
	static ref ACCESS_FLAGS: Vec<(u16, &'static str)> = {
		vec![
		(0x0001, "PUBLIC"), // Declared public; may be accessed from outside its package.
		(0x0010, "FINAL"), // Declared final; no subclasses allowed.
		(0x0020, "SUPER"), // Treat superclass methods specially when invoked by the invokespecial instruction.
		(0x0200, "INTERFACE"), // Is an interface, not a class.
		(0x0400, "ABSTRACT"), // Declared abstract; must not be instantiated.
		(0x1000, "SYNTHETIC"), // Declared synthetic; not present in the source code.
		(0x2000, "ANNOTATION"), // Declared as an annotation type.
		(0x4000, "ENUM"), // Declared as an enum type.
		(0x8000, "MODULE"), // Is a module, not a class or interface.
		]
	};
}

pub fn read_access(reader: &mut Reader) -> ReadResult {
	let bytes = reader.read_2u()?;
	let flags = to_u16(bytes);
	print_bytes(0, bytes);
	print!("Flags:");
	for &(flag, name) in ACCESS_FLAGS.iter() {
		if (flags & flag) != 0 {
			print!(" {}", name);
		}
	}
	println!("");

	Ok(())
}