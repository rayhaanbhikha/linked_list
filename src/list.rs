use crate::list_permutation::ListPermutation;
use crate::node::Node;
use std::fmt::Display;

pub struct List<T>
where
    T: Display + Default,
{
    start: Node<T>,
}

impl<T> List<T>
where
    T: Display + Default,
{
    pub fn new() -> Self {
        let dummy_node = Node::new(Default::default());
        Self { start: dummy_node }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.start.get(index + 1)
    }

    pub fn add(&mut self, val: T) {
        self.start.add(val)
    }

    pub fn delete(&mut self, index: usize) {
        self.start.delete(index + 1)
    }

    pub fn iter<'a>(&'a self) -> ListPermutation<'a, T> {
        ListPermutation::new(self.start.next.as_ref())
    }
}

impl<T> Display for List<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for val in self.iter() {
            write!(f, "{} -> ", val)?
        }
        Ok(())
    }
}
