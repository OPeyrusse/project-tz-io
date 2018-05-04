pub fn print_bytes(indent: u8, bytes: &[u8]) {
	for _i in 0..indent {
		print!("  ");
	}
	for b in bytes {
		match *b {
			// Small fix as it is not possible to put trailing 0s in front of hexa
			b @ 0 ... 9 => print!("0{:X} ", b),
			_ => print!("{:X} ", b)
		}
	}
	print!("> ")
}