use std::collections::HashSet;
use std::ops::FnOnce;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};
use parser::instruction::{ValuePointer, Operation};
use checker::CheckResult;

/// Module checking that the ports referenced by inputs
/// or outputs for duplicated port numbers.

fn check_ports<T, F: Fn(&T) -> u32>(
    result: &mut CheckResult,
    inputs: &Vec<T>, 
    accessor: F) -> HashSet<u32> {
  let mut values = HashSet::new();
  let mut duplicates = HashSet::new();
  for port in inputs.iter() {
    let value = accessor(port);
    if !values.insert(value) {
      duplicates.insert(value);
    }
  }
  duplicates
}

// fn collect_output_ports(outputs: &Vec<OutputMapping>) -> HashSet<u32> {
//   outputs.iter().map(|ref output| output.from).collect::<HashSet<u32>>()
// }

fn dups_to_str(duplicates: HashSet<u32>) -> String {
  duplicates.iter().fold(String::new(), |mut acc, value| {
    acc.push_str(format!("{},", value).as_str());
    acc
  })
}

fn check_node(node: &NodeBlock, result: &mut CheckResult) {
  let input_duplicates = check_ports(result, &node.1, |ref input| input.to);
  if !input_duplicates.is_empty() {
    result.add_error(format!(
      "Duplicated input ports referenced in {}: {}",
      &node.0, dups_to_str(input_duplicates)));
  }

  let output_duplicates = check_ports(result, &node.2, |ref output| output.from);
  if !output_duplicates.is_empty() {
    result.add_error(format!(
      "Duplicated output ports referenced in {}: {}",
      &node.0, dups_to_str(output_duplicates)));
  }
}

pub fn check(tree: &ParsingTree, result: &mut CheckResult) -> bool {
  let initial_count = result.error_count();
  for node in tree {
    check_node(node, result);
  } 

  initial_count == result.error_count()
}

#[cfg(test)]
mod tests {
  use super::*;

  
}