use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            next: self.head.take(),
            elem,
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Prevent possible stack overflow caused by the default drop behavior (recursively)
        // in case of many linked list elements. This is done by iteratively dropping
        // every member of the list.
        let mut link = mem::replace(&mut self.head, None);
        while let Some(mut node) = link {
            link = mem::replace(&mut node.next, None)
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    elem: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list: List<usize> = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
