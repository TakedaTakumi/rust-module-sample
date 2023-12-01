mod domain;
use domain::{entity::Node, value_object::ID};

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
