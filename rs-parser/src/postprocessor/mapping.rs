use std::collections::HashMap;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};

type Index<'a> = HashMap<&'a String, usize>;
fn map_node_to_idx(parsingTree: &ParsingTree, index: &mut Index) {
  let mut index = HashMap::new();
  for (i, &(ref node, _, _, _)) in parsingTree.iter().enumerate() {
    if let &Node::Node(ref nodeId) = node {
      index.insert(nodeId, i);
    }
  }
}

fn complete_outputs(parsingTree: &mut ParsingTree, index: &Index) {
  let mut additions = Vec::new();
  for mut node in parsingTree.iter() {
    // Read inputs and add them to the source
    let inputs: &Vec<InputMapping> = &node.1;
    for input in inputs.iter() {
      if let Node::Node(ref nodeId) = input.from.node {
        match index.get(nodeId) {
          Some(idx) => {
            let output = &parsingTree[*idx];
            let addtional_output = complete_output(output, input.from.port, nodeId, input.to);
            if let Some(o) = addtional_output {
              additions.push((*idx, o));
            }
          },
          _ => panic!("No reference to node {}", nodeId)
        }
      }
    }
  }

  for (i, output) in additions {
    let output_node = &mut parsingTree[i];
    output_node.2.push(output);
  }
}

fn complete_output(node: &NodeBlock, from: u32, targetId: &String, to: u32) -> Option<OutputMapping> {
  // Skip if the port is already present
  let outputs: &Vec<OutputMapping> = &node.2;
  if !outputs.iter().any(|output| match output.to.node {
    Node::Node(ref id) => id == targetId && output.to.port == to,
    _ => false
  }) {
    Some(OutputMapping { 
      from: from, 
      to: Port { 
        node: Node::new_node(targetId.as_str()), 
        port: to 
      }
    })
  } else {
    None
  }
}

pub fn complete_mappings(parsingTree: &mut ParsingTree) {
  let mut nodes = HashMap::new();
  {
    map_node_to_idx(parsingTree, &mut nodes);
  }

  {
    // Iterate over the nodes to complete the definitions
    complete_outputs(parsingTree, &nodes);
  }
}