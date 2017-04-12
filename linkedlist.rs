struct Node {
    next: Option<Box<Node>>,
    data: [u8],
}

impl Node {
    fn new(initial_data: [u8]) -> Box<Node> {
        Box::new(Node {
            next: None,
            data: initial_data
        })
    }

    fn add(&mut self, new_data: [u8]) {
        if let Some(_) = self.next { unimplemented!(); }
        self.next = Some(Box::new(Node {
            next: None,
            data: new_data
        }));
    }
}

fn main() {
    let mut head = Node::new(b"my fascinating data");
}
