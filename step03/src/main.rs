mod domain;
use domain::{entity::node::Node, value_object::id::ID};

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
