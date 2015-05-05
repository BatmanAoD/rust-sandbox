// A simple vector-like structure for ints, implemented as a linked-list of arrays

// Data

struct Node
{
    data_seg_: [i32; 64],
    next_: Option<Box<Node>>,
    size_: i32,
}

pub struct IntVector
{
    head_: Box<Node>,
}

// Requests

impl std::ops::IndexMut<i32> for Node
{
    fn index_mut(&'a mut self, index: i32) -> &'a mut i32
    {
        self.data_seg_[index]
    }
}

impl Node
{
}

