use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(node) => 1 + node.count(),
        }
    }

    pub fn push(&mut self, element: T) {
        let mut new_node = Node::new(element);
        if self.head.is_some() {
            let node = self.head.take().unwrap();
            new_node.next_node = Some(node);
        }
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.take()?;
        if head.next_node.is_some() {
            let head_next = head.next_node.take().unwrap();
            self.head = Some(head_next);
        }
        Some(head.element)
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.element),
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        if self.head.is_some() {
            let mut possible_head = self.head.take().unwrap();
            while possible_head.next_node.is_some() {
                let possible_head_temp = possible_head.next_node.take().unwrap();
                possible_head.next_node = self.head;
                self.head = Some(possible_head);
                possible_head = possible_head_temp;
            }
            possible_head.next_node = self.head;
            self.head = Some(possible_head);
        }

        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for i in iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec![];
        while let Some(e) = linked_list.pop() {
            vec.push(e);
        }
        vec.reverse();
        vec
    }
}

struct Node<T> {
    element: T,
    next_node: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self {
            element,
            next_node: None,
        }
    }

    fn count(&self) -> usize {
        match &self.next_node {
            None => 0,
            Some(node) => 1 + node.count(),
        }
    }
}
