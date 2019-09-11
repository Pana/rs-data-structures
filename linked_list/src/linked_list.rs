use crate::LinkedList;

struct Node<T> {
    val: T,
    next: Box<Option<Node<T>>>,
}

struct BasicLinkedList<T> {
    head: Option<Node<T>>,
}

impl <T> BasicLinkedList<T> {
    fn new() -> Self {
        BasicLinkedList{
            head: None,
        }
    }
}

impl <T> LinkedList<T> for BasicLinkedList<T> {
    fn is_empty(&self) -> bool {
        match &self.head {
            None => true,
            Some(_node) => false,
        }
    }

    fn clear(&self) {

    }

    fn len(&self) -> usize {
        0
    }

    fn print(&self) {

    }

    fn insert(&mut self, val: T, index: usize) -> bool {
        true
    }

    fn remove(&mut self, val: T, index: usize) -> bool {
        true
    }

    fn search(&self, val: T) -> bool {
        true
    }
}