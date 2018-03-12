use std::collections::HashMap;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};

type Index<'a> = HashMap<&'a String, usize>;
fn map_node_to_idx<'a>(parsingTree: &'a ParsingTree, index: &mut Index<'a>) {
  for (i, &(ref node, _, _, _)) in parsingTree.iter().enumerate() {
    if let &Node::Node(ref nodeId) = node {
      index.insert(nodeId, i);
    }
  }
}

fn complete_inputs(parsingTree: &mut ParsingTree, index: &Index) {
  let mut additions = Vec::new();
  for mut node in parsingTree.iter() {
    // Read outputs and add them to their sources
    let outputs: &Vec<OutputMapping> = &node.2;
    for output in outputs.iter() {
      if let Node::Node(ref nodeId) = output.to.node {
        match index.get(nodeId) {
          Some(idx) => {
            let input_node = &parsingTree[*idx];
            let addtional_input = complete_input(input_node, nodeId, output.to.port, output.from);
            if let Some(i) = addtional_input {
              additions.push((*idx, i));
            }
          },
          _ => panic!("No reference to node {}", nodeId)
        }
      }
    }
  }

  for (i, output) in additions {
    let output_node = &mut parsingTree[i];
    output_node.1.push(output);
  }
}

fn complete_input(node: &NodeBlock, targetId: &String, from: u32, to: u32) -> Option<InputMapping> {
  // Skip if the port is already present
  let inputs: &Vec<InputMapping> = &node.1;
  if !inputs.iter().any(|input| match input.from.node {
    Node::Node(ref id) => id == targetId && input.from.port == from,
    _ => false
  }) {
    Some(InputMapping {  
      from: Port { 
        node: Node::new_node(targetId.as_str()), 
        port: from 
      },
      to: to 
    })
  } else {
    None
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
            let output_node = &parsingTree[*idx];
            let addtional_output = complete_output(output_node, input.from.port, nodeId, input.to);
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
  // {
  //   map_node_to_idx(&parsingTree, &mut nodes);
  // }

  {
    complete_inputs(parsingTree, &nodes);
  }

  {
    complete_outputs(parsingTree, &nodes);
  }
}