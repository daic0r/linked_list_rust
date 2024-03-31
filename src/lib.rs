use std::marker::PhantomData;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>
}

struct LinkedList<T: Default> {
    nodes: Vec<Box<Node<T>>>,
    head: *mut Node<T>,
    tail: *mut Node<T>,
    length: usize
}

impl<T: Default> LinkedList<T> {
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
            if  self.tail != std::ptr::null_mut() {
                (*self.tail).next = nod_ptr;
            }
        }
        if self.head == std::ptr::null_mut() {
            self.head = nod_ptr;
        }
        self.tail = nod_ptr;
        self.length += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let mut nod = Box::new(Node::<T>{
            value,
            next: self.head,
            prev: std::ptr::null_mut()
        });
        let nod_ptr = &mut *nod as *mut Node<T>;
        unsafe {
            if  self.head != std::ptr::null_mut() {
                (*self.head).prev = nod_ptr;
            }
        }
        if self.tail == std::ptr::null_mut() {
            self.tail = nod_ptr;
        }
        self.head = nod_ptr;
        self.length += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail == std::ptr::null_mut() {
            assert!(self.length == 0);
            return None;
        }
        let ret;
        unsafe {
            if (*self.tail).prev != std::ptr::null_mut() {
                (*(*self.tail).prev).next = std::ptr::null_mut();
            }
            ret = Some(std::mem::take(&mut (*self.tail).value));
            // Check if only 1 element
            if self.head == self.tail {
                assert!(self.length == 1);
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
            } else {
                self.tail = (*self.tail).prev;
            }
        }
        self.length -= 1;
        ret
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head == std::ptr::null_mut() {
            assert!(self.length == 0);
            return None;
        }
        let ret;
        unsafe {
            if (*self.head).next != std::ptr::null_mut() {
                (*(*self.head).next).prev = std::ptr::null_mut();
            }
            ret = Some(std::mem::take(&mut (*self.head).value));
            // Check if only 1 element
            if self.head == self.tail {
                assert!(self.length == 1);
                self.head = std::ptr::null_mut();
                self.tail = std::ptr::null_mut();
            } else {
                self.head = (*self.head).next;
            }
        }
        self.length -= 1;
        ret
    }

    pub fn len(&self) -> usize {
        self.length
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

struct LinkedListIterator<'a, T: Default> {
    node: *mut Node<T>,
    phantom: PhantomData<&'a T>
}

impl<'a, T: Default> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node == std::ptr::null_mut() {
            return None;
        }
        unsafe {
            let ret = &(*self.node).value;
            self.node = (*self.node).next;
            Some(ret)
        }
    }
}

struct LinkedListIteratorMut<'a, T: Default> {
    node: *mut Node<T>,
    phantom: PhantomData<&'a T>
}

impl<'a, T: Default> Iterator for LinkedListIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node == std::ptr::null_mut() {
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
}
