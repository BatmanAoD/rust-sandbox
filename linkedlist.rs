struct Node {
    next: Option<Box<Node>>,
    data: [u8],
}

impl Node {
    fn new(initial_data: &[u8]) -> Box<Node> {
        // Turns out I don't know how to create new DST's.
        // See: http://stackoverflow.com/a/25753422/1858225
        let mut head = Box::new(Node {
            next: None,
            data: [0; initial_data.len()]
        });
        head.data.copy_from_slice(initial_data);
    }

    fn add(&mut self, new_data: &[u8]) {
        if let Some(_) = self.next { unimplemented!(); }
        self.next = new(new_data);
    }
}

fn main() {
    let mut head = Node::new(b"my fascinating data");
}
