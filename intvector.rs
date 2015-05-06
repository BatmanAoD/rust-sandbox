// A simple vector-like structure for ints, implemented as a linked-list of arrays

use std::ops::{Index, IndexMut};

// Data

struct Node
{
    // In the original "Belle" implementation, this is a Box (to demonstrate the syntax of boxed
    // arrays), but that is unnecessary.
    data_seg_: [i32; 64],
    next_: Option<Box<Node>>,
    // TODO use an iterator rather than raw size value?
    size_: usize,
}

pub struct IntVector
{
    head_: Box<Node>,
    // User iterators instead?
    first_: usize,
    last_: usize,
    size_: usize,
}

// Node Requests

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

    fn initNext(&mut self)
    {
        self.next_ = Some(Box::new(Node::new()));
    }

    fn isFull(&self) -> bool
    {
        self.size_ == 64
    }

    // Provide access to `next_` via a fxn?
}


// IntVector Requests

impl std::ops::IndexMut<usize> for IntVector
{
    fn index_mut<'a>(&'a mut self, index: usize) -> &mut i32
    {
        let seg_no = (index - self.start_) / 64;
        let index = (index - self.start_) % 64;
        let containing_seg = self.traverse(
            |seg| ! seg.isFull,
            |seg| if (seg.next.is_none()) {seg.initNext;}
        );

        &mut self.containing_seg[index]
    }
}

impl std::ops::Index<usize> for IntVector
{
    type Output = i32;
    fn index<'a>(&'a self, index: usize) -> &i32
    {
        & self.data_seg_[index]
    }
}

impl IntVector
{
    fn traverse<'a>(&'a mut self, cond: Fn, op: Fn) -> &'a mut Node
    {
        let seg_iter = self.head_;
        // XXX TODO perform iteration
        seg_iter
    }
}
