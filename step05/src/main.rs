mod domain;
use domain::{
    entity::Node,
    value_object::{IdCollection, ID},
};

fn main() {
    let list = IdCollection::new();
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
    println!("list: {:?}", list);
}
