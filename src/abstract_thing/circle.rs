

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    fn set_next(&mut self, next: Rc<RefCell<Node>>) {
        self.next = Some(next);
    }
}

#[test]
fn test_circle() {
    let first = Rc::new(RefCell::new(Node::new(1)));
    let second = Rc::new(RefCell::new(Node::new(2)));
    let third = Rc::new(RefCell::new(Node::new(3)));

    first.borrow_mut().set_next(second.clone());
    second.borrow_mut().set_next(third.clone());
    third.borrow_mut().set_next(first.clone());

    // 现在 first -> second -> third -> first，形成环形链表
}
