
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
            Empty => panic!("Attempt to extract value from Empty Node."),
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
        let mut node  = Box::new(Node::new(value));
        let     front = take(self);
        node.next     = front;
        *self         = Filled(node);
    }
    pub fn push_back(&mut self, value: T) {
        let mut curr = self;
        while !curr.is_empty() {
            curr = &mut curr.next;
        }
        *curr = Self::from_value(value);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let ret;
        if !self.is_empty() && !self.next.is_empty() {
            let next = self.next.take();
            ret      = Some(self.take().extract_value());
            *self    = next;
        } else {
            ret = Some(self.take().extract_value());
        }
        ret
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let mut curr = self;
        while !curr.is_empty() && !curr.next.is_empty() { 
            curr = &mut curr.next; 
        }
        if !curr.is_empty() {
             Some(curr.take().extract_value())
        } else {
            None
        }
    }
    pub fn insert(&mut self, index: usize, value: T) {
        let mut node = Self::from_value(value);
        let mut curr = self;
        let mut i    = 0;
        while !curr.is_empty() && i + 1 < index {
            curr = &mut curr.next;
            i   += 1;
        }
        let next  = curr.next.take();
        node.next = next;
        curr.next = node;
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let mut ret  = None;
        let mut curr = self;
        let mut i    = 0;
        while !curr.is_empty() && i + 1 < index {
            curr = &mut curr.next;
            i   += 1;
        }
        if !curr.is_empty() {
            let mut next = curr.next.take();
            
            if !next.next.is_empty() {
                curr.next = next.next.take();
                ret       = Some(next.extract_value());
            } 
        }
        ret
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


pub struct LinkedListIterMut<'a, T>(&'a mut LinkedList<T>);

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {              
            self.0  = &mut self.0.next;
            let ret = Some(&mut self.0.value); 
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
/*
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item     = &'a mut T;
    type IntoIter = LinkedListIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterMut(self)
    }
}*/

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

impl<T> Deref for LinkedList<T> {
    type Target = Node<T>;
    fn deref(&self) -> &Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference an empty list."),
        }
    }
}
impl<T> DerefMut for LinkedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Filled(node) => node,
            Empty => panic!("Attempt to dereference an empty list."),
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
        self.get(i).expect(&format!("Index value {} out of range.", i))
    }
}
impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.get_mut(i).expect(&format!("Index value {} out of range.", i))
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
        let mut list = (0..5).collect::<LinkedList<usize>>();
        assert_eq!(list.remove(3).unwrap(), 3);
        assert_eq!(list.get(3).unwrap(), &4);
    }
}








