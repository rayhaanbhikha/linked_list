# linked_list

```rs
use linked_list::list::List;

fn main() {
    let mut list = List::new();

    list.add("S");
    list.add("R");
    list.add("A");

    println!("{:?}", list.get(1)); // Some("R")

    println!("{}", list); // S -> R -> A ->

    list.delete(1);

    println!("{:?}", list.get(1)); // Some("A")
    
    println!("{}", list); // S -> A ->
}
```
