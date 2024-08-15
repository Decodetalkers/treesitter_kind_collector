use treesitter_type_collector::tree_sitter_consts;

#[tree_sitter_consts("asserts/node-types.json")]
struct NodeTypes;

fn main() {
    println!("{}", NodeTypes::SYS_ARGUMENT);
    println!("{:?}", NodeTypes::NODE_TYPES);
}
