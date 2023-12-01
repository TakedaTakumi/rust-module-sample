#[derive(Debug)]
struct ID {
    #[allow(dead_code)]
    value: String,
}
impl ID {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    id: ID,
    #[allow(dead_code)]
    label: String,
}
impl Node {
    fn new(id: ID, label: &str) -> Self {
        Self {
            id,
            label: label.to_string(),
        }
    }
}

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
