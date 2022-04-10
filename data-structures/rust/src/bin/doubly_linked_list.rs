use std::{
    fmt::{Display, Formatter, Result, Debug},
    usize, rc::Rc, borrow::{BorrowMut, Borrow},
};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Debug> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    count: usize,
}

impl<T: Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    fn insert_last(&mut self, v: T) {
        let mut new_node = Rc::new(Node::new(v));
        match self.tail {
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            },
            Some(tail) => {
                self.tail.borrow_mut().unwrap().next = Some(Box::new());
                new_node.prev = Some(Rc::clone(&tail));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
        self.count += 1;
    }

    fn display(&self) {
        if self.count < 1 {
            println!("Linked List is empty !!");
            return;
        }

        println!(
            "Current length: {}, Head: {:?}, Tail: {:?}",
            self.count, self.head, self.tail
        );
    }
}

fn main() {
    let mut ll: LinkedList<i32> = LinkedList::new();

    ll.insert_last(50);

    ll.display();
}
