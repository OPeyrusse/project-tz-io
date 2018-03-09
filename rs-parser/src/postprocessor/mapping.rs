use std::collections::HashMap;

use parser::ParsingResult;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};

fn complete_output(node: &mut NodeBlock, from: u32, targetId: &String, to: u32) {
  // Skip if the port is already present
  let outputs = node.2;
  if !outputs.iter().any(|output| match output.to.node {
    &Node::Node(ref id) => id == targetId && output.to.port == to,
    _ => false
  }) {
    outputs.push(OutputMapping { 
      from: from, 
      to: Port { 
        node: Node::new_node(targetId.as_str()), 
        port: to 
      }
    });
  }
}

pub fn complete_mappings(parsingTree: &mut ParsingResult) {
  let mut nodes = HashMap::new();
  for (i, &(node, _, _, _)) in parsingTree.unwrap().iter().enumerate() {
    if let Node::Node(ref nodeId) = node {
      nodes.insert(nodeId, i);
    }
  }

  // Iterate over the nodes to complete the definitions
  for node in parsingTree.iter() {
    // Read inputs and add them to the source
    let inputs = node.1;
    for input in inputs.iter() {
      if let Node::Node(ref nodeId) = input.from.node {
        let mut node = parsingTree.unwrap()[nodes.get(nodeId).unwrap()];
        complete_output(node, input.from.port, nodeId, input.to);
      }
    }

    // Read outputs and add them to the target
  }

}