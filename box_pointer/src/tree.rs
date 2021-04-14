#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: T,
    right: Box<Option<Node<T>>>,
    left: Box<Option<Node<T>>>,
}

impl <T: Eq+Ord>Node<T> {
    pub fn new(value: T) -> Node<T> {
        return Node {
            value: value,
            right: Box::new(None),
            left: Box::new(None),
        };
    }

    pub fn add(&mut self, value: T) {
        if self.value == value {
            println!("This value is already present.");
        } else if value < self.value {
            let node_option = &mut *self.left;
            if let Some(node) = node_option {
                node.add(value);
            } else {
                self.left = Box::new(Some(Node::new(value)));
            }
        } else {
            let node_option = &mut *self.right;
            if let Some(node) = node_option {
                node.add(value);
            } else {
                self.right = Box::new(Some(Node::new(value)));
            }
        }
    }
}