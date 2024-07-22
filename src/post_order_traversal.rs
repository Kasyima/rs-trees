// Post-order traversal starts from the left subtree, followed by the right subtree,
// and finally the root node.

use std::fmt::Debug;
use crate::{BinaryTree, Link};

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    fn postorder(&self) {
        match &self.left { 
            Some(node) => node.postorder(),
            None => (),
        }
        match &self.right { 
            Some(node) => node.postorder(),
            None => (),
        }
        println!("key: {:?}", &self.key);
    }
}

// post-order: implemented externally [by recursion]
fn postorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
    }
}