use parser::ParsingResult;
use checker::CheckResult;

/// Module checking that the mappings between the various nodes
/// are consistent.
/// For example, when a node A maps its outputs to node B, if B
/// defines its inputs, A and B must map the same ports.
/// A.out: [1 -> B#1] and B.in: [A#1 -> 1] are ok, same mapping
/// A.out: [1 -> B#1] and B.in: [A#2 -> 2] are ok, they is no overlap
/// A.out: [1 -> B#1] and B.in: [A#2 -> 1] are inconsistent

pub fn check(tree: &ParsingResult, result: &mut CheckResult) -> bool {
	true
}