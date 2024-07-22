// Pre-order traversal starts from the root node, then moves to the left subtree,
// and finally the right subtree.

// To traverse a tree algorithmically using pre-order traversal, recursively call the pre-order traversal
// method on the left subtree from the root node.

// Can be implemented as an internal method or an external function.

use std::fmt::Debug;
use crate::{BinaryTree, Link};

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    fn preorder(&self) {
        println!("key: {:?}", &self.key);
        match &self.left { 
            Some(node) => node.preorder(),
            None => (),
        }
        match &self.right { 
            Some(node) => node.preorder(),
            None => (),
        }
    }
}

// pre-order: implemented externally [by recursion]
fn preorder<T: Clone + Ord + ToString + Debug> (bt: Link<T>) {
    if !bt.is_none() {
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}