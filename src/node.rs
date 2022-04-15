use std::fmt::Display;

#[derive(Default)]
pub struct Node<T>
where
    T: Display,
{
    value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Display + Default,
{
    pub fn new(val: T) -> Self {
        Self {
            value: val,
            next: None,
        }
    }

    pub fn get_val(&self) -> &T {
        &self.value
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_node(index, 0)
    }

    fn get_node(&self, index: usize, current_index: usize) -> Option<&T> {
        if index == current_index {
            Some(&self.value)
        } else if let Some(ref next_node) = self.next {
            next_node.get_node(index, current_index + 1)
        } else {
            None
        }
    }

    pub fn delete(&mut self, index: usize) {
        self.delete_node(index, 0);
    }

    fn delete_node(&mut self, index: usize, current_index: usize) {
        if index == (current_index + 1) {
            // take ownership of the next node (node that will be deleted).
            if let Some(node_to_delete) = std::mem::take(&mut self.next) {
                self.next = node_to_delete.next;
            }
        } else if let Some(next_node) = &mut self.next {
            next_node.delete_node(index, current_index + 1)
        }
    }

    pub fn add(&mut self, val: T) {
        self.add_node(Node::new(val))
    }

    fn add_node(&mut self, new_node: Node<T>) {
        if let Some(next_node) = &mut self.next {
            next_node.add_node(new_node);
        } else {
            self.next = Some(Box::new(new_node));
        }
    }
}

// // TODO: not sure about this.
// impl<T> Display for Node<T>
// where
//     T: Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "{}", &self.value)?;
//         let mut current_node = self.next.as_ref();

//         loop {
//             if let Some(n) = current_node {
//                 writeln!(f, "{}", n.value)?;
//                 current_node = n.next.as_ref();
//             } else {
//                 break;
//             }
//         }

//         Ok(())
//     }
// }
