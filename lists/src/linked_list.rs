use std::cell::RefCell;
use std::rc::Rc;
type SingleLink = Option<Rc<RefCell<Node>>>;
struct Node {
    value: i32,
    next: SingleLink,
}
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}
impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}
impl TransactionLog {
    fn new() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
}
