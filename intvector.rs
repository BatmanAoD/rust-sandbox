// A simple vector-like structure for ints, implemented as a linked-list of arrays

use std::ops::{Index, IndexMut};

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
}

