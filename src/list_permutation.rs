use std::fmt::Display;

use crate::node::Node;

pub struct ListPermutation<'a, T>
where
    T: Display + Default,
{
    current_node: Option<&'a Box<Node<T>>>,
}

impl<'a, T> ListPermutation<'a, T>
where
    T: Display + Default,
{
    pub fn new(starting_node: Option<&'a Box<Node<T>>>) -> Self {
        Self {
            current_node: starting_node,
        }
    }
}

impl<'a, T> Iterator for ListPermutation<'a, T>
where
    T: Display + Default,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_node) = &self.current_node {
            let val = current_node.get_val();
            self.current_node = current_node.next.as_ref();
            Some(val)
        } else {
            None
        }
    }
}
