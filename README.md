# rust-linked-list
An example of a singly linked list in Rust. No `unsafe` code blocks. No hacks. A straightforward implementation.

Lists items can be accessed by index with `.get()` or array index convention, `list[0]`. 

It implements `FromIterator` and `From<Iterator>`.

```rust
let     list = (0..5).collect::<LinkedList<usize>>();
let mut list = LinkedList::from(0..5);

assert_eq!(list.get(0), Some(&0));

list.push_front(42);
list.push_back(99);
assert_eq!(list.pop_front(), Some(42));
assert_eq!(list.pop_back(), Some(99));
list.insert(2, 22);
assert_eq!(list.remove(0), Some(0));

for n in &list {
    print!("{}", n);
}
assert_eq!(list[1], &2);

```
It's a good start, and has many features, but it's nowhere as complete as objects in `std::collections`. 
Feel free to modify and extend it if you find it useful.

The approach that I found simplifies implementing node based data structures in Rust ivolves a tight relationship 
between the nodes of the structure and the structure itself, along with the `Deref` and `DerefMut` traits.
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

impl<T> Deref for LinkedList<T> {
    type Target = Node<T>;
    fn deref(&self) -> &Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference 
                             an empty list."),
        }
    }
}
```
