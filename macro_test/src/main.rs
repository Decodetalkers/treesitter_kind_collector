use treesitter_type_collector::treesitter_consts;
treesitter_consts!("../../asserts/node-types.json");

fn main() {
    println!("{}", node_consts::SYS_ARGUMENT);
    println!("{:?}", node_consts::NODE_TYPES);
}
