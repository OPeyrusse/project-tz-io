use flags::to_method_access;
use pool::{PoolList, resolve_utf8_value};
use printer::print_bytes;
use reader::{Reader, ReadResult, to_u16};

fn read_access(reader: &mut Reader, indent: u8) -> ReadResult {
	let bytes = reader.read_2u()?;
	print_bytes(indent, bytes);

	let flags = to_method_access(to_u16(bytes));
	print!("Flags:");
	for flag in &flags {
		print!(" {}", flag);
	}
	println!("");

	Ok(())
}

fn read_method_name(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	let bytes = reader.read_2u()?;
	print_bytes(indent, bytes);

	let name_idx = to_u16(bytes);
	let method_name = resolve_utf8_value(pool, name_idx as usize)
		.expect("Method name not in the constant pool");

	println!("Method '{}'", method_name);

	Ok(())
}

fn read_method(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	read_access(reader, indent)?;
	read_method_name(reader, pool, indent)
}

pub fn read(reader: &mut Reader, pool: &PoolList) -> ReadResult {
	let count: u16;
	{
		let bytes = reader.read_2u()?;
		print_bytes(0, bytes);

		count = to_u16(bytes);
		println!("Method count = {}", count);
	}

	for _i in 0..count {
		read_method(reader, pool, 1)?;
	}

	Ok(())
}