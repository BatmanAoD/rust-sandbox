struct Node {
    next: Option<Box<Node>>,
}

impl Node {
    fn new() -> Box<Node> {
        Box::new(Node {
            next: None
        })
    }
}

fn main() {
    let mut head = Box::new(Node { next: None });
    head.next = Some(Node::new());
    head = head.next.unwrap();
}

/* FULL VERSION:

struct Node {
    next: Option<Box<Node>>,
    data: u8,
}

impl Node {
    fn new(initial_data: u8) -> Box<Node> {
        Box::new(Node {
            next: None,
            data: initial_data
        })
    }

    fn add(&mut self, new_data: u8) {
        if let Some(_) = self.next { unimplemented!(); }
        self.next = Some(Node::new(new_data));
    }
}

fn main() {
    let mut head = Node::new(b'a');
    println!("Initial head: {}", head.data);
    head.add(b'b');
    println!("Tail: {}", (*head.next.as_ref().unwrap()).data);
    head = head.next.unwrap();
    println!("New head: {}", head.data);
}

*/
