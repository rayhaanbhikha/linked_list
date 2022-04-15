mod node;

use self::node::Node;
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
}

#[cfg(test)]
mod tests {
    use super::List;

    fn generate_list(vals: &[&str]) -> List<String> {
        let mut list = List::new();

        for val in vals.iter() {
            list.add((*val).to_owned());
        }

        list
    }

    #[test]
    fn successfully_add_nodes() {
        let list = generate_list(&["a", "b", "c", "d"]);

        assert_eq!(list.get(0), Some(&"a".to_owned()), "has a");
        assert_eq!(list.get(1), Some(&"b".to_owned()), "has b");
        assert_eq!(list.get(2), Some(&"c".to_owned()), "has c");
        assert_eq!(list.get(3), Some(&"d".to_owned()), "has d");
    }

    #[test]
    fn delete_node_at_end() {
        let chars = &["a", "b", "c", "d"];
        let mut list = generate_list(chars);

        list.delete(chars.len() - 1);

        assert_eq!(list.get(0), Some(&"a".to_owned()), "has a");
        assert_eq!(list.get(1), Some(&"b".to_owned()), "has b");
        assert_eq!(list.get(2), Some(&"c".to_owned()), "has c");
        assert_eq!(list.get(3), None, "has no d");
    }

    #[test]
    fn delete_node_at_start() {
        let chars = &["a", "b", "c", "d"];
        let mut list = generate_list(chars);

        list.delete(0);

        assert_eq!(list.get(0), Some(&"b".to_owned()), "has b");
        assert_eq!(list.get(1), Some(&"c".to_owned()), "has c");
        assert_eq!(list.get(2), Some(&"d".to_owned()), "has d");
    }
}
