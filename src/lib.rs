mod list;
mod list_permutation;
mod node;

#[cfg(test)]
mod tests {
    use super::list::List;

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

    #[test]
    fn iterate_over_list() {
        let chars = vec!["a", "b", "c", "d"];
        let list = generate_list(&chars);
        let mut iterator = list.iter();

        for char in chars.iter() {
            let x = Some((*char).to_owned());
            assert_eq!(iterator.next(), x.as_ref())
        }
    }

    #[test]
    fn print_expected_list() {
        let list = generate_list(&["a", "b", "c", "d"]);

        assert_eq!(format!("{}", list), "a -> b -> c -> d -> ")
    }
}
