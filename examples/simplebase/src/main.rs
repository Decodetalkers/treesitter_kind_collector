use treesitter_kind_collector::tree_sitter_kinds;

#[tree_sitter_kinds("asserts/node-types.json")]
struct NodeKindTypes;

fn main() {
    println!("{}", NodeKindTypes::ARGUMENT);
    println!("{:?}", NodeKindTypes::NODE_TYPES);
}
