use std::collections::HashMap;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};

type Index<'a> = HashMap<&'a String, usize>;
fn map_node_to_idx<'a>(tree: &'a ParsingTree, index: &mut Index<'a>) {
  for (i, &(ref node, _, _, _)) in tree.iter().enumerate() {
    if let &Node::Node(ref nodeId) = node {
      index.insert(nodeId, i);
    }
  }
}

fn complete_inputs(mut tree: ParsingTree, index: &Index) -> ParsingTree {
  let mut additions = Vec::new();
  for node in tree.iter() {
    // Read outputs and add them to their sources
    let outputs: &Vec<OutputMapping> = &node.2;
    for output in outputs.iter() {
      if let Node::Node(ref node_id) = output.to.node {
        match index.get(node_id) {
          Some(idx) => {
            let input_node = &tree[*idx];
            let addtional_input = complete_input(input_node, node_id, output.to.port, output.from);
            if let Some(i) = addtional_input {
              additions.push((*idx, i));
            }
          },
          _ => panic!("No reference to node {}", node_id)
        }
      }
    }
  }

  for (i, output) in additions {
    let output_node = &mut tree[i];
    output_node.1.push(output);
  }

  tree
}

fn complete_input(node: &NodeBlock, target_id: &String, from: u32, to: u32) -> Option<InputMapping> {
  // Skip if the port is already present
  let inputs: &Vec<InputMapping> = &node.1;
  if !inputs.iter().any(|input| match input.from.node {
    Node::Node(ref id) => id == target_id && input.from.port == from,
    _ => false
  }) {
    Some(InputMapping {
      from: Port {
        node: Node::new_node(target_id.as_str()),
        port: from
      },
      to: to
    })
  } else {
    None
  }
}

fn complete_outputs(mut tree: ParsingTree, index: &Index) -> ParsingTree{
  let mut additions = Vec::new();
  for node in tree.iter() {
    // Read inputs and add them to the source
    let inputs: &Vec<InputMapping> = &node.1;
    for input in inputs.iter() {
      if let Node::Node(ref node_id) = input.from.node {
        match index.get(node_id) {
          Some(idx) => {
            let output_node = &tree[*idx];
            let addtional_output = complete_output(output_node, input.from.port, node_id, input.to);
            if let Some(o) = addtional_output {
              additions.push((*idx, o));
            }
          },
          _ => panic!("No reference to node {}", node_id)
        }
      }
    }
  }

  for (i, output) in additions {
    let output_node = &mut tree[i];
    output_node.2.push(output);
  }

  tree
}

fn complete_output(node: &NodeBlock, from: u32, target_id: &String, to: u32) -> Option<OutputMapping> {
  // Skip if the port is already present
  let outputs: &Vec<OutputMapping> = &node.2;
  if !outputs.iter().any(|output| match output.to.node {
    Node::Node(ref id) => id == target_id && output.to.port == to,
    _ => false
  }) {
    Some(OutputMapping {
      from: from,
      to: Port {
        node: Node::new_node(target_id.as_str()),
        port: to
      }
    })
  } else {
    None
  }
}

pub fn complete_mappings(tree: ParsingTree) -> ParsingTree {
  let mut nodes = HashMap::new();
  {
    map_node_to_idx(&tree, &mut nodes);
  }

  let tree = complete_inputs(tree, &nodes);
  let tree = complete_outputs(tree, &nodes);

  tree
}