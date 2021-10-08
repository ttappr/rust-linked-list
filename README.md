# rust-linked-list
An example of a singly linked list in Rust. No `unsafe` code blocks. No hacks. A straightforward implementation.

Each node of the linked list is both a `Node` and a `LinkedList`. The variant `Empty` represents an empty list, 
and the `Filled` variant is a list with nodes.

```rust
pub struct Node<T> {
    value : T,
    next  : LinkedList<T>,
}

pub enum LinkedList<T> {
    Empty,
    Filled(Box<Node<T>>),
}
```

Lists items can be accessed by index with `.get()` or array index convention, `list[0]`. 

It implements `FromIterator` and `From<Iterator>`.

```rust
let list = (0..5).collect::<LinkedList<usize>>();
let list = LinkedList::from(0..5);
```
It's a good start, and has many features, but it's nowhere as complete as objects in `std::collections`. 
Feel free to make extensions. Feel free to modify and extend it if you find it useful.
