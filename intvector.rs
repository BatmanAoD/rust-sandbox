// A simple vector-like structure for ints, implemented as a linked-list of arrays

use std::ops::{Index, IndexMut};

// Data

struct Node
{
    data_seg_: [i32; 64],
    next_: Option<Box<Node>>,
    // TODO use an iterator rather than raw size value?
    size_: usize,
}

pub struct IntVector
{
    head_: Box<Node>,
}

// Requests

impl std::ops::IndexMut<usize> for Node
{
    fn index_mut<'a>(&'a mut self, index: usize) -> &mut i32
    {
        &mut self.data_seg_[index]
    }
}

impl std::ops::Index<usize> for Node
{
    type Output = i32;
    fn index<'a>(&'a self, index: usize) -> &i32
    {
        & self.data_seg_[index]
    }
}

impl Node
{
// Provide access to `next_` via a fxn?

    fn new() -> Node
    {
        Node {
            // Possible ot leave this uninitialized? There's no reason to 0-out the elements that
            // aren't yet "active".
            data_seg_: [0; 64],
            next_: None,
            size_: 0
        }
    }


    // This fxn is only called if Node's segment is NOT full
    fn append(&mut self, n: i32)
    {
        // TODO Is it possible to do this without introducing a new binding?
        let new_index = self.size_;
        self[new_index] = n;
        self.size_ += 1;
    }

    fn init_next(&mut self)
    {
        self.next_ = Some(Box::new(Node::new()));
    }

}

