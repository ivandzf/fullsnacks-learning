use std::{
    fmt::{Debug, Display, Formatter, Result},
    usize,
};

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Debug> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.next {
            None => write!(f, "{:?}", self.value),
            Some(ref node) => write!(f, "{:?},{}", self.value, *node),
        }
    }
}

impl<T> Node<T> {
    fn new(v: T) -> Self {
        Node {
            value: v,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Node<T>>,
    count: usize,
}

impl<T: Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = write!(f, "[");
        match self.head {
            None => {}
            Some(ref node) => {
                let _ = write!(f, "{}", node);
            }
        }
        write!(f, "]")
    }
}

impl<T: Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            count: 0,
        }
    }

    // need traverse operation to get the tail, O(n)
    fn insert_last(&mut self, v: T) {
        let new_node = Node::new(v);
        match self.head {
            None => self.head = Some(new_node),
            Some(_) => {
                self.get_next_nth_node(self.count - 1)
                    .map(|node| node.next = Some(Box::new(new_node)));
            }
        }

        // manually count, to reduce calculate the lenght of linked list
        self.count = self.count + 1
    }

    // directly set on the head, O(1)
    fn insert_first(&mut self, v: T) {
        let mut new_node = Node::new(v);
        match self.head {
            None => self.head = Some(new_node),
            Some(_) => {
                let current_head = self.head.take().unwrap();
                new_node.next = Some(Box::new(current_head));
                self.head = Some(new_node);
            }
        }

        // manually count, to reduce calculate the lenght of linked list
        self.count = self.count + 1
    }

    // need traverse operation to get the tail, O(n)
    fn remove_last(&mut self) {
        match &self.head {
            None => {}
            Some(_) => {
                if self.count == 1 {
                    self.head = None;
                } else {
                    self.get_next_nth_node(self.count - 2)
                        .map(|node| node.next = None);
                }
                // self.get_next_nth_node(self.count - 1).take();
                // manually count, to reduce calculate the lenght of linked list
                self.count = self.count - 1
            }
        }
    }

    // directly take on the head, O(1)
    fn remove_first(&mut self) {
        self.head
            .take()
            .map(|head| self.head = head.next.map(|node| *node));
        // manually count, to reduce calculate the lenght of linked list
        self.count = self.count - 1
    }

    // traverse and get the linked list by number of index
    fn get_next_nth_node(&mut self, n: usize) -> Option<&mut Node<T>> {
        let mut nth_node = self.head.as_mut();
        for _ in 0..n {
            nth_node = match nth_node {
                None => return None,
                Some(node) => node.next.as_mut().map(|node| &mut **node),
            }
        }

        nth_node
    }

    fn display(&self) {
        if self.head.is_none() {
            println!("Linked List is empty !!");
            return;
        }

        println!(
            "Current length: {}, Linked List: {:?}",
            self.count,
            self.head.as_ref().unwrap()
        );
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.insert_last(1);
    ll.insert_last(2);
    ll.insert_last(5);

    ll.display();

    ll.insert_first(20);

    ll.display();

    ll.remove_first();

    ll.display();

    ll.remove_last();

    ll.display();
}
