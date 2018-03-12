use std::collections::HashMap;

use parser::ParsingTree;
use parser::address::{Node, Port};
use parser::syntax::{NodeBlock, InputMapping, OutputMapping};
use checker::CheckResult;

/// Module checking that the mappings between the various nodes
/// are consistent.
/// For example, when a node A maps its outputs to node B, if B
/// defines its inputs, A and B must map the same ports.
/// A.out: [1 -> B#1] and B.in: [A#1 -> 1] are ok, same mapping
/// A.out: [1 -> B#1] and B.in: [A#2 -> 2] are ok, they is no overlap
/// A.out: [1 -> B#1] and B.in: [A#2 -> 1] are inconsistent

type Index<'a> = HashMap<&'a String, usize>;
// TODO move this method to some utility module
fn map_node_to_idx<'a>(parsingTree: &'a ParsingTree, index: &mut Index<'a>) {
  for (i, &(ref node, _, _, _)) in parsingTree.iter().enumerate() {
    if let &Node::Node(ref nodeId) = node {
      index.insert(nodeId, i);
    }
  }
}

pub fn check(tree: &ParsingTree, result: &mut CheckResult) -> bool {
  let mut index = HashMap::new();
  {
    map_node_to_idx(&tree, &mut index);
  }

	true
}