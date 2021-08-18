use std::rc::Rc;

use markup5ever_rcdom::Node;

pub fn find_first_in_node<T>(node: &Rc<Node>, f: &dyn Fn(&Rc<Node>) -> Option<T>) -> Option<T> {
    match f(node) {
        Some(x) => Some(x),
        None => {
            for child in node.children.borrow().iter() {
                match find_first_in_node(child, f) {
                    Some(x) => return Some(x),
                    None => {}
                }
            }

            None
        }
    }
}
