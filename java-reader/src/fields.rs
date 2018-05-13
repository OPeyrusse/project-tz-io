use printer::print_bytes;
use reader::{Reader, ReadResult, to_u16};

pub fn read(reader: &mut Reader) -> ReadResult {
	let count: u16;
	{
		let bytes = reader.read_2u()?;
		count = to_u16(bytes);
		print_bytes(0, bytes);
		println!("Field count = {}", count);
	}

	if count > 0 {
		panic!("Class defining {} field(s). Not supporting this", count);
	}

	Ok(())
}