use std::collections::HashSet;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{InputMapping, OutputMapping};
use checker::CheckResult;

fn dups_to_str(duplicates: HashSet<u32>) -> String {
  duplicates.iter().fold(String::new(), |mut acc, value| {
    acc.push_str(format!("{},", value).as_str());
    acc
  })
}

fn check_inputs(tree: &ParsingTree, result: &mut CheckResult) {
  let mut input_ports = HashSet::new();
  let mut duplicates = HashSet::new();
  for node in tree {
    let inputs = &node.1;
    for input in inputs {
      let node = &input.from.node;
      let port = input.from.port;
      if node == &Node::In && !input_ports.insert(port) {
        duplicates.insert(port);
      }
    }
  }

  if !duplicates.is_empty() {
    result.add_error(format!(
      "Duplicated uses of input ports {}",
      dups_to_str(duplicates)));
  }

  // TODO look for min and max and compare to the size of the ports to find unsued ports
}

fn check_outputs(tree: &ParsingTree, result: &mut CheckResult) {
  let mut output_ports = HashSet::new();
  let mut duplicates = HashSet::new();
  for node in tree {
    let outputs = &node.2;
    for output in outputs {
      let node = &output.to.node;
      let port = output.to.port;
      if node == &Node::Out && !output_ports.insert(port) {
        duplicates.insert(port);
      }
    }
  }

  if !duplicates.is_empty() {
    result.add_error(format!(
      "Duplicated uses of output ports {}",
      dups_to_str(duplicates)));
  }
}

pub fn check(tree: &ParsingTree, result: &mut CheckResult) -> bool {
  let initial_count = result.error_count();
  check_inputs(tree, result);
  check_outputs(tree, result);

	result.error_count() == initial_count
}