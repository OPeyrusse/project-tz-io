mod mapping;
mod instruction;

use std::result::Result;

use parser::ParsingResult;

pub struct CheckResult {
	warnings: Vec<String>,
	errors: Vec<String>
}

impl CheckResult {
	pub fn new() -> CheckResult {
		CheckResult { warnings: Vec::new(), errors: Vec::new() }
	}

	pub fn add_warning(&mut self, message: String) {
		self.warnings.push(message);
	}

	fn has_errors(&self) -> bool {
		!self.errors.is_empty()
	}

	fn has_warnings(&self) -> bool {
		!self.warnings.is_empty()
	}
}

pub fn check(parsing_tree: &ParsingResult) {
	match parsing_tree {
		&Result::Ok(ref res) => {
			println!("{:?}", res);
			let mut checks = CheckResult::new();
			if !mapping::check(res, &mut checks) {
				println!(" -> Mapping errors ...")
			}
			if !instruction::check(res, &mut checks) {
				println!(" -> Instruction errors ...")
			}
			if checks.has_warnings() {
				println!("Warnings in your project");
			}
			if checks.has_errors() {
				println!("Errors in your project");
				panic!("Exit after errors");
			}
		},
		&Result::Err(ref e) => println!("Failure: {:?}", e)
	}
}