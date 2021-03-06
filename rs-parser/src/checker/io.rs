use std::collections::HashSet;

use parser::address::Node;
use parser::ParsingTree;
use checker::CheckResult;

fn dups_to_str(duplicates: HashSet<u32>) -> String {
  duplicates.iter().fold(String::new(), |mut acc, value| {
    acc.push_str(format!("{},", value).as_str());
    acc
  })
}

/// Checks that all ports from 1 to max are used,
/// otherwise, we are having holes in our input/output array.
/// This returns the list of ports not used.
/// 
/// # Arguments
/// * `ports` - set of used ports
fn check_ranges(ports: &HashSet<u32>) -> Option<HashSet<u32>> {
  let max_port = ports.iter().max().unwrap_or(&0u32);
  if (*max_port as usize) > ports.len() {
    let missing_ports = (1..(max_port + 1)).filter(|v| !ports.contains(v)).collect();
    Some(missing_ports)
  } else {
    None
  }
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

  if let Some(ununsed_ports) = check_ranges(&input_ports) {
    result.add_warning(format!(
      "Unused ports in the input: {}",
      dups_to_str(ununsed_ports)));
  }
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

  if let Some(ununsed_ports) = check_ranges(&output_ports) {
    result.add_warning(format!(
      "Unused ports in the output: {}",
      dups_to_str(ununsed_ports)));
  }
}

pub fn check(tree: &ParsingTree, result: &mut CheckResult) -> bool {
  let initial_count = result.error_count();
  check_inputs(tree, result);
  check_outputs(tree, result);

	result.error_count() == initial_count
}

#[cfg(test)]
mod tests {
  use super::*;
  
  use parser::address::Port;
  use parser::syntax::{InputMapping, OutputMapping};

  #[test]
  fn test_check_in_ok() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 1
            },
            to: 1
          }
        ],
        vec![],
        vec![]
      ),
      (
        Node::new_node(&"b"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 2
            },
            to: 2
          }
        ],
        vec![],
        vec![]
      )
    ];
    check_inputs(&nodes, &mut checks);
    assert_eq!(checks.has_errors(), false);
  }

  #[test]
  fn test_check_in_ko() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 3
            },
            to: 1
          }
        ],
        vec![],
        vec![]
      ),
      (
        Node::new_node(&"b"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 3
            },
            to: 2
          }
        ],
        vec![],
        vec![]
      )
    ];
    check_inputs(&nodes, &mut checks);
    assert_eq!(checks.has_errors(), true);
  }

  #[test]
  fn test_check_out_ok() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![],
        vec![
          OutputMapping {
            from: 1,
            to: Port {
              node: Node::Out,
              port: 1
            }
          }
        ],
        vec![]
      ),
      (
        Node::new_node(&"b"),
        vec![],
        vec![
          OutputMapping {
            from: 2,
            to: Port {
              node: Node::Out,
              port: 2
            }
          }
        ],
        vec![]
      )
    ];
    check_outputs(&nodes, &mut checks);
    assert_eq!(checks.has_errors(), false);
  }

  #[test]
  fn test_check_out_ko() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![],
        vec![
          OutputMapping {
            from: 1,
            to: Port {
              node: Node::Out,
              port: 3
            }
          }
        ],
        vec![]
      ),
      (
        Node::new_node(&"b"),
        vec![],
        vec![
          OutputMapping {
            from: 2,
            to: Port {
              node: Node::Out,
              port: 3
            }
          }
        ],
        vec![]
      )
    ];
    check_outputs(&nodes, &mut checks);
    assert_eq!(checks.has_errors(), true);
  }

  #[test]
  fn test_check_complete() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 3
            },
            to: 1
          }
        ],
        vec![
          OutputMapping {
            from: 1,
            to: Port {
              node: Node::Out,
              port: 3
            }
          }
        ],
        vec![]
      ),
      (
        Node::new_node(&"b"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 3
            },
            to: 2
          }
        ],
        vec![
          OutputMapping {
            from: 2,
            to: Port {
              node: Node::Out,
              port: 3
            }
          }
        ],
        vec![]
      )
    ];
    check(&nodes, &mut checks);
    assert_eq!(checks.has_errors(), true);
    assert_eq!(checks.error_count(), 2);
  }

  #[test]
  fn test_warnings_about_unused_inputs() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![
          InputMapping {
            from: Port {
              node: Node::In,
              port: 3
            },
            to: 1
          }
        ],
        vec![],
        vec![]
      )
    ];
    check(&nodes, &mut checks);
    assert_eq!(checks.has_warnings(), true);
  }

  #[test]
  fn test_warnings_about_unused_outputs() {
    let mut checks = CheckResult::new();
    let nodes = vec![
      (
        Node::new_node(&"a"),
        vec![],
        vec![
          OutputMapping {
            from: 1,
            to: Port {
              node: Node::Out,
              port: 3
            }
          }
        ],
        vec![]
      )
    ];
    check(&nodes, &mut checks);
    assert_eq!(checks.has_warnings(), true);
  }

}