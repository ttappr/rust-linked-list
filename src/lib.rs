
//! An example of a singly linked list in Rust. The goal was to implement this
//! in straightforward Rust without resorting to any hackish solutions like
//! blocks of `unsafe` just to evade the borrow checker.

use std::iter::FromIterator;
use std::mem::take;
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};

use LinkedList::*;

/// The node of the list. Holds the values of the list and a field for the
/// next item.
///
#[derive(Debug)]
pub struct Node<T> {
    value : T,
    next  : LinkedList<T>,
}

impl<T> Node<T> {
    /// Creates a new node with the given value. Private method.
    ///
    fn new(value: T) -> Self {
        Node { value, next: Empty }
    }
}

/// Represents a list. Each node is essentially a `LinkedList` linked to 
/// subsequent lists. `LinkedList` has two variants: `Empty` indicating the
/// list is empty, and `Filled` when populated.
///
#[derive(Debug)]
pub enum LinkedList<T> {
    Empty,
    Filled(Box<Node<T>>),
}

impl<T> LinkedList<T> {
    /// Create a new LinkedList.
    ///
    pub fn new() -> Self {
        Empty
    }
    
    /// Create a new LinkedList (or think of it as a node), with the given
    /// value.
    ///
    pub fn from_value(value: T) -> Self {
        Filled(Box::new(Node::new(value)))
    }
    
    /// Returns `true` if the list is empty.
    ///
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
    
    /// Returns a reference to the value at the given 0-based index in the list.
    /// An `O(n)` operation.
    ///
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut i   = 0;
        let mut ret = None;
        for value in self {
            i += 1;
            if i > index {
                ret = Some(value);
                break;
            }
        }
        ret
    }
    
    /// Returns a mutable reference to the value held by the LinkedList node at
    /// the given index.
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut i    = 0;
        let mut curr = self;
        while !curr.is_empty() && i + 1 < index {
            i += 1;
            curr = &mut curr.next;
        }
        if !curr.is_empty() {
            Some(&mut curr.value)
        } else {
            None
        }
    }
    
    /// Pushes the given value at the front of the list. An `O(1)` operation.
    ///
    pub fn push_front(&mut self, value: T) {
        let mut node  = LinkedList::from_value(value);
        let     front = take(self);
        node.next     = front;
        *self         = node;
    }
    
    /// Pushes the given value onto the end of the list. An `O(n)` operation.
    ///
    pub fn push_back(&mut self, value: T) {
        let mut curr = self;
        while !curr.is_empty() {
            curr = &mut curr.next;
        }
        *curr = Self::from_value(value);
    }
    
    /// Removes the first LinkedList node from the front of the list. If there
    /// isn't one, `None` is returned. An `O(1)` operation.
    ///
    pub fn pop_front(&mut self) -> Option<T> {
        let mut ret = None;
        if !self.is_empty() {
            if !self.next.is_empty() {
                let next = self.next.take();
                ret      = Some(self.take().extract_value());
                *self    = next;
            } else {
                ret = Some(self.take().extract_value());
            }
        }
        ret
    }
    
    /// Removes the last item in the LinkedList if present. Returns the value
    /// of the item as `Some(value)`, or if the list was empty, `None` is
    /// returned. An `O(n)` operation.
    ///
    pub fn pop_back(&mut self) -> Option<T> {
        let mut ret = None;
        if !self.is_empty() {
            let mut curr = self;
            while !curr.next.is_empty() { 
                curr = &mut curr.next; 
            }
            if !curr.is_empty() {
                 ret = Some(curr.take().extract_value());
            }
        }
        ret
    }
    
    /// Inserts the given value into the list at the position given by `index`.
    /// If the index is greater than the length of the list, it is placed at
    /// the end of the list. An `O(n)` operation.
    ///
    pub fn insert(&mut self, index: usize, value: T) {
        let mut node = LinkedList::from_value(value);
        if self.is_empty() || index == 0 {
            if !self.is_empty() {
                node.next = self.take();
            }
            *self = node;
        } else {
            let mut prev = self;
            let mut i    = 1;
            while !prev.next.is_empty() && i < index {
                prev = &mut prev.next;
                i   += 1;
            }
            node.next = prev.next.take();
            prev.next = node;
        }
    }
    
    /// Removes the value at the given `index` and returns it. If one wasn't
    /// at the given index, the last item in the list is returned as 
    /// `Some(value)`. If the list is empty, then `None` is returned.
    /// An `O(n)` operation.
    ///
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut node = Empty;
        if self.is_empty() || index == 0 {
            if !self.is_empty() {
                node  = self.take();
                *self = node.next.take();
            }
        } else {
            let mut prev = self;
            let mut i    = 1;
            while !prev.next.is_empty() && i < index {
                prev = &mut prev.next;
                i   += 1;
            }
            if !prev.next.is_empty() {
                node      = prev.next.take();
                prev.next = node.next.take();
            }
        }
        if !node.is_empty() { Some(node.extract_value()) } 
        else                { None                       }
    }
     
    /// Internal method that returns the contents of the current 
    /// node/LinkedList, replacing it with `Empty`.
    ///
    fn take(&mut self) -> LinkedList<T> {
        take(self)
    }
    
    /// Returns the non-reference value of the LinkedList node. Ownership is 
    /// transferred to the caller. Internal method.
    ///
    fn extract_value(self) -> T {
        match self {
            Filled(bx) => bx.value,
            Empty => panic!("Attempt to extract value 
                             from Empty Node."),
        }
    }
}

/// Represents a list iterator. Holds a reference to a linked list for 
/// iteration over its elements.
///
pub struct LinkedListIter<'a, T>(&'a LinkedList<T>);

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    
    /// Returns the next element in the list if there is one as `Some(value)`.
    /// Returns `None` when the end of the list is reached.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {              
            let ret = Some(&self.0.value); 
            self.0  = &self.0.next;
            ret
        }
    }
}

/// Trait that facilitates the creation of list iterators via `.into_iter()`.
///
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item     = &'a T;
    type IntoIter = LinkedListIter<'a, T>;
    
    /// Returns an iterator to the list reference. `self` should be a reference
    /// to a `LinkedList`.
    ///
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self)
    }
}

/// Trait that facilitates the creation of a list using the `.collect()` method
/// provided by other iterators.
///
impl<T> FromIterator<T> for LinkedList<T> {

    /// Returns a new list containing the elements the iterator provides.
    ///
    fn from_iter<I>(iter: I) -> Self 
    where
        I: IntoIterator<Item=T>,
    {
        let mut list = LinkedList::new();
        for item in iter.into_iter() {
            list.push_back(item);
        }
        list
    }
}


/// Trait that allows an iterator to be converted to a list using `.from()`.
///
impl<T, I> From<I> for LinkedList<T> 
where
    I: Iterator<Item=T>,
{
    /// Returns a list instance from the provided iterator.
    ///
    fn from(iter: I) -> Self {
        iter.collect()
    }
}

/// Trait that allows other code to transparently access the `Node` instance 
/// within a `LinkedList`.
///
impl<T> Deref for LinkedList<T> {
    type Target = Node<T>;
    
    /// Method called to resolve a list reference to a `Node`. Will panic if
    /// the list is `Empty`.
    ///
    fn deref(&self) -> &Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference 
                             an empty list."),
        }
    }
}

/// Mutable dereference trait that provides transparent access to the `Node`
/// within a `LinkedList`.
///
impl<T> DerefMut for LinkedList<T> {

    /// Invoked to resolve a list to its internal node.
    ///
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference 
                             an empty list."),
        }
    }
}

/// Provides a default value for a `LinkedList`. This trait comes into play
/// when operations like `take()` are executed on an object.
///
impl<T> Default for LinkedList<T> {

    /// Returns the default value for a list, which is `Empty`.
    ///
    fn default() -> Self {
        Empty
    }
}

/// Trait that gives the lists support for array indexing syntax.
///
impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    
    /// Invoked hwere array indexing is used within source code elsewhere.
    ///
    fn index(&self, i: usize) -> &Self::Output {
        self.get(i)
            .unwrap_or_else(|| panic!("Index value {} 
                                       out of range.", i))
    }
}

/// Trait that gives lists array indexing support to access mutable references
/// to list items.
///
impl<T> IndexMut<usize> for LinkedList<T> {

    /// Returns a mutable reference to the value at the given index.
    ///
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.get_mut(i)
            .unwrap_or_else(|| panic!("Index value {}
                                       out of range.", i))
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;
    
    #[test]
    fn get() {
        let list = (0..5).collect::<LinkedList<usize>>();
        assert_eq!(list.get(3).unwrap(), &3);
    }
    #[test]
    fn remove() {
        let mut list = (0..=5).collect::<LinkedList<usize>>();
        assert_eq!(list.remove(3), Some(3));
        assert_eq!(list.get(3), Some(&4));
        assert_eq!(list.remove(3), Some(4));
        assert_eq!(list.remove(3), Some(5));
        
        list = (0..1).collect::<LinkedList<usize>>();
        assert_eq!(list.remove(0), Some(0));
        assert_eq!(list.remove(0), None);
    }
    #[test]
    fn insert() {
        let mut list = LinkedList::from(0..5);
        list.insert(1, 9);
        assert_eq!(list.get(1), Some(&9));
        assert_eq!(list.get(2), Some(&1));
        list = LinkedList::new();
        list.insert(0, 9);
        assert_eq!(list.get(0), Some(&9));
        list.insert(0, 8);
        assert_eq!(list.get(0), Some(&8));
        assert_eq!(list.get(1), Some(&9));
        assert_eq!(list.get(2), None);
    }
    #[test]
    fn push_front() {
        let mut list = LinkedList::new();
        list.push_front(5);
        assert_eq!(list.get(0), Some(&5));
        list.push_front(4);
        assert_eq!(list.get(0), Some(&4));
        assert_eq!(list.get(1), Some(&5));
        assert_eq!(list.get(2), None);
    }
    #[test]
    fn push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), None);
    }
    #[test]
    fn pop_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None);
        list = LinkedList::from(0..3);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn leet_1() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_back(3);
        list.insert(1, 2);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        list.remove(0);
        assert_eq!(list.get(0), Some(&2));
    }
}








