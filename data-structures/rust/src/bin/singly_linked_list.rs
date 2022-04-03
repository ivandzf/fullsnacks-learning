use std::{
    fmt::{Debug, Display, Formatter, Result},
    usize,
};

#[derive(Debug)]
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
    fn push(&mut self, v: T) {
        match self.head {
            None => self.head = Some(Node::new(v)),
            Some(_) => {
                let last_node = self.get_nth_node(self.count - 1);
                last_node.map(|node| node.next = Some(Box::new(Node::new(v))));
            }
        }

        // manually count, to reduce calculate the lenght of linked list
        self.count = self.count + 1
    }

    fn pop(&mut self) {
        match self.head {
            None => {}
            Some(_) => {
                let mut last_node = self.get_nth_node(self.count - 1);
                last_node.take();
            }
        }
    }

    // traverse and get the linked list by number of index
    fn get_nth_node(&mut self, n: usize) -> Option<&mut Node<T>> {
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

        println!("{:?}", self.head.as_ref().unwrap());
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push(1);
    ll.push(2);
    ll.push(5);

    ll.display();

    ll.pop();

    ll.display();

    ll.push(10);

    ll.display();
}
