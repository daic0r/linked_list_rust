use std::marker::PhantomData;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>
}

pub struct LinkedList<T: Default + PartialEq> {
    nodes: Vec<Box<Node<T>>>,
    head: *mut Node<T>,
    tail: *mut Node<T>,
    length: usize
}

impl<T: Default + PartialEq> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::<T> {
            nodes: vec![],
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            length: 0
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut nod = Box::new(Node::<T>{
            value,
            next: std::ptr::null_mut(),
            prev: self.tail
        });
        let nod_ptr = &mut *nod as *mut Node<T>;
        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = nod_ptr;
            }
        }
        if self.head.is_null() {
            self.head = nod_ptr;
        }
        self.tail = nod_ptr;
        self.length += 1;
        self.nodes.push(nod);
    }

    pub fn push_front(&mut self, value: T) {
        let mut nod = Box::new(Node::<T>{
            value,
            next: self.head,
            prev: std::ptr::null_mut()
        });
        let nod_ptr = &mut *nod as *mut Node<T>;
        unsafe {
            if ! self.head.is_null() {
                (*self.head).prev = nod_ptr;
            }
        }
        if self.tail.is_null() {
            self.tail = nod_ptr;
        }
        self.head = nod_ptr;
        self.length += 1;
        self.nodes.push(nod);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            assert!(self.length == 0);
            return None;
        }
        let ret;
        let del_ptr;
        unsafe {
            if !(*self.tail).prev.is_null() {
                (*(*self.tail).prev).next = std::ptr::null_mut();
            }
            ret = Some(std::mem::take(&mut (*self.tail).value));
            del_ptr = self.tail;
            // Check if only 1 element
            if self.head == self.tail {
                assert!(self.length == 1);
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
            } else {
                self.tail = (*self.tail).prev;
            }
        }
        self.nodes.retain(|x| (&**x as *const Node<T>) != del_ptr);
        self.length -= 1;
        assert!(self.length == self.nodes.len());
        ret
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            assert!(self.length == 0);
            return None;
        }
        let ret;
        let del_ptr;
        unsafe {
            if !(*self.head).next.is_null() {
                (*(*self.head).next).prev = std::ptr::null_mut();
            }
            ret = Some(std::mem::take(&mut (*self.head).value));
            del_ptr = self.head;
            // Check if only 1 element
            if self.head == self.tail {
                assert!(self.length == 1);
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
            } else {
                self.head = (*self.head).next;
            }
        }
        self.nodes.retain(|x| (&**x as *const Node<T>) != del_ptr);
        self.length -= 1;
        assert!(self.length == self.nodes.len());
        ret
    }

    pub fn front(&self) -> Option<&T> {
        if let Some(f) = self.front_mut() {
            return Some(f);
        }
        None
    }

    pub fn back(&self) -> Option<&T> {
        if let Some(b) = self.back_mut() {
            return Some(b);
        }
        None
    }

    pub fn back_mut(&self) -> Option<&mut T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            Some(&mut (*self.tail).value)
        }
    }

    pub fn front_mut(&self) -> Option<&mut T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            Some(&mut (*self.head).value)
        }
    }

    fn remove_impl<F>(&mut self, pred: F) -> Option<T>
        where F: Fn(&T) -> bool
    {
        if self.length == 0 {
            return None;
        }

        let mut ptr = self.head;
        let mut ret: Option<T> = None;

        unsafe {
            while !ptr.is_null() && !pred(&(*ptr).value) {
                ptr = (*ptr).next;
            }
        }

        if ptr.is_null() {
            return ret;
        }

        if ptr == self.head {
            ret = self.pop_front();
        }
        else if ptr == self.tail {
            ret = self.pop_back();
        }
        else {
            unsafe {
                if !(*ptr).prev.is_null() {
                    (*(*ptr).prev).next = (*ptr).next;
                }
                if !(*ptr).next.is_null() {
                    (*(*ptr).next).prev = (*ptr).prev;
                }
                ret = Some(std::mem::take(&mut (*ptr).value));
            }
            self.length -= 1;
            self.nodes.retain(|x| &**x as *const Node<T> != ptr);
        }

        ret
    }

    pub fn remove_all<F>(&mut self, pred: F)
        where F: Fn(&T) -> bool
    {
        while self.remove_impl(&pred).is_some() {}
    }

    pub fn remove_element(&mut self, element: &T) -> Option<T> {
        self.remove_impl(|x| x == element)
    }

    pub fn element_position(&self, element: &T) -> Option<usize> {
        for (index, i) in self.iter().enumerate() {
            if i == element {
                return Some(index);
            }
        }
        None
    }

    pub fn insert(&mut self, index: usize, element: T) {
        if index == 0 {
            self.push_front(element);
            return;
        }
        if index == self.length {
            self.push_back(element);
            return;
        }
        let mut ptr = self.head;
        for _ in 0..index {
            unsafe {
                ptr = (*ptr).next;
            }
        }
        let mut nod;
        unsafe {
            nod = Box::new(Node::<T>{
                value: element,
                next: ptr,
                prev: (*ptr).prev
            });
            let nod_ptr = &mut *nod as *mut Node<T>;
            (*(*ptr).prev).next = nod_ptr;
            (*ptr).prev = nod_ptr;
        }
        self.nodes.push(nod);
        self.length += 1;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::<T> {
            node: self.head,
            phantom: PhantomData,
        }
    }

    pub fn iter_mut(&self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut::<T> {
            node: self.head,
            phantom: PhantomData,
        }
    }
}

pub struct LinkedListIterator<'a, T: Default> {
    node: *mut Node<T>,
    phantom: PhantomData<&'a T>
}

impl<'a, T: Default> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_null() {
            return None;
        }
        unsafe {
            let ret = &(*self.node).value;
            self.node = (*self.node).next;
            Some(ret)
        }
    }
}

pub struct LinkedListIteratorMut<'a, T: Default> {
    node: *mut Node<T>,
    phantom: PhantomData<&'a T>
}

impl<'a, T: Default> Iterator for LinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_null() {
            return None;
        }
        unsafe {
            let ret = &mut (*self.node).value;
            self.node = (*self.node).next;
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.len(), 0);
        list.push_back(1);
        assert_eq!(list.len(), 1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        list.push_back(3);
        assert_eq!(list.len(), 3);
        list.pop_back();
        assert_eq!(list.len(), 2);
        list.pop_back();
        assert_eq!(list.len(), 1);
        list.pop_back();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_front() {
        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.front(), None);
        list.push_back(1);
        assert_eq!(list.front(), Some(&1));
        list.push_back(2);
        assert_eq!(list.front(), Some(&1));
        list.pop_front();
        assert_eq!(list.front(), Some(&2));
        list.pop_front();
        assert_eq!(list.front(), None);
    }

    #[test]
    fn test_back() {
        let mut list = LinkedList::<i32>::new();
        assert_eq!(list.back(), None);
        list.push_back(1);
        assert_eq!(list.back(), Some(&1));
        list.push_back(2);
        assert_eq!(list.back(), Some(&2));
        list.pop_back();
        assert_eq!(list.back(), Some(&1));
        list.pop_back();
        assert_eq!(list.back(), None);
    }

    #[test]
    fn test_remove_all() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        list.push_back(7);
        list.push_back(8);
        list.push_back(9);
        list.push_back(10);
        list.remove_all(|x| *x % 2 == 0);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_remove_element() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.remove_element(&2);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);

        list.remove_element(&1);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);

        list.remove_element(&4);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        list.remove_element(&3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.insert(1, 4);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        list.insert(0, 5);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        list.insert(5, 6);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&6));
    }
}
