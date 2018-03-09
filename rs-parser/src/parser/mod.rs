pub mod address;
pub mod syntax;
pub mod common;
pub mod instruction;

use nom::{IResult, error_to_list};
use std::result::Result;

pub type ParsingResult = Result<Vec<syntax::NodeBlock>, ()>;

// fn print_errors(e: &ErrorKind) {
//   let mut first = true;
//   let errors = vec![e];
//   print!("Error while parsing: ");
//   for error in &errors {
//     if first {
//       first = false;
//     } else {
//       print!("  caused by: ");
//     }
//     println!("{:?}", error);
//   }
// }

pub fn parse(input: &common::RawData) -> ParsingResult {
  let res = syntax::node_list(input);
  match res {
    IResult::Done(_i, o) => Result::Ok(o),
    IResult::Error(e) => {
      // match &e {
      //   ErrorKind => print_errors(&e),
      //   Err => println!("{:?}", e),
      //   _ => println!("{:?}", e)
      // }

      let mut first = true;
      println!("{:?}", e);
      let errors = error_to_list(&e);
      for error in &errors {
        if first {
          println!("Error while parsing: {:?}", error);
          first = false;
        } else {
          println!("  caused by: {:?}", error);
        }
      }
      // println!("{:?}", e);
      Result::Err(())
    },
    IResult::Incomplete(needed) => {
      println!("Missing data. Needed {:?}", needed);
      Result::Err(())
    }
  }
}