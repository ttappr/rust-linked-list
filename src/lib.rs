
use std::iter::FromIterator;
use std::mem::take;
use std::ops::{Deref, DerefMut};
use std::ops::{Index, IndexMut};

use LinkedList::*;


#[derive(Debug)]
pub struct Node<T> {
    value : T,
    next  : LinkedList<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: Empty }
    }
}

#[derive(Debug)]
pub enum LinkedList<T> {
    Empty,
    Filled(Box<Node<T>>),
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Empty
    }
    pub fn from_value(value: T) -> Self {
        Filled(Box::new(Node::new(value)))
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
    fn take(&mut self) -> LinkedList<T> {
        take(self)
    }
    fn extract_value(self) -> T {
        match self {
            Filled(bx) => bx.value,
            Empty => panic!("Attempt to extract value 
                             from Empty Node."),
        }
    }
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
    pub fn push_front(&mut self, value: T) {
        let mut node  = LinkedList::from_value(value);
        let     front = take(self);
        node.next     = front;
        *self         = node;
    }
    pub fn push_back(&mut self, value: T) {
        let mut curr = self;
        while !curr.is_empty() {
            curr = &mut curr.next;
        }
        *curr = Self::from_value(value);
    }
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
}

pub struct LinkedListIter<'a, T>(&'a LinkedList<T>);

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
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


impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item     = &'a T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self)
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
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

impl<T, I> From<I> for LinkedList<T> 
where
    I: Iterator<Item=T>,
{
    fn from(iter: I) -> Self {
        iter.collect()
    }
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
impl<T> DerefMut for LinkedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference 
                             an empty list."),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Empty
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        self.get(i)
            .unwrap_or_else(|| panic!("Index value {} 
                                       out of range.", i))
    }
}
impl<T> IndexMut<usize> for LinkedList<T> {
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








