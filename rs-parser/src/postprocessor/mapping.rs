use std::collections::HashMap;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};

type Index<'a> = HashMap<&'a String, usize>;
fn map_node_to_idx<'a>(parsingTree: &'a ParsingTree) -> Index<'a> {
  let mut nodes = HashMap::new();
  for (i, &(ref node, _, _, _)) in parsingTree.iter().enumerate() {
    if let &Node::Node(ref nodeId) = node {
      nodes.insert(nodeId, i);
    }
  }
  nodes
}

fn complete_outputs(parsingTree: &mut ParsingTree, index: &Index) {
  for mut node in parsingTree.iter() {
    // Read inputs and add them to the source
    let inputs: &Vec<InputMapping> = &node.1;
    for input in inputs.iter() {
      if let Node::Node(ref nodeId) = input.from.node {
        match index.get(nodeId) {
          Some(idx) => {
            let mut output = &mut parsingTree[*idx];
            complete_output(&mut output, input.from.port, nodeId, input.to);
          },
          _ => panic!("No reference to node {}", nodeId)
        }
      }
    }
  }
}

fn complete_output(node: &mut NodeBlock, from: u32, targetId: &String, to: u32) {
  // Skip if the port is already present
  let mut outputs: &mut Vec<OutputMapping> = &mut node.2;
  if !outputs.iter().any(|output| match output.to.node {
    Node::Node(ref id) => id == targetId && output.to.port == to,
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

pub fn complete_mappings(parsingTree: &mut ParsingTree) {
  let nodes = map_node_to_idx(parsingTree);

  {
    // Iterate over the nodes to complete the definitions
    complete_outputs(parsingTree, &nodes);
  }
}