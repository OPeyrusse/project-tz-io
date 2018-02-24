mod address;
mod syntax;
mod common;
mod instruction;

use nom::IResult;
use std::result::Result;

pub fn parse(input: &common::RawData) -> Result<Vec<syntax::NodeBlock>, ()> {
  let res = syntax::node_list(input);
  match res {
    IResult::Done(_i, o) => Result::Ok(o),
    IResult::Error(e) => {
      println!("error: {:?}", e);
      Result::Err(())
    },
    IResult::Incomplete(needed) => {
      println!("Missing data. Needed {:?}", needed);
      Result::Err(())
    }
  }
}