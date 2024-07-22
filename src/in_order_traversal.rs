// In-order traversal starts from the left subtree, followed by the root node,
// and finally the right subtree.

use std::fmt::Debug;
use crate::{BinaryTree, Link};

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T>  {
    fn inorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().inorder();
        }
        println!("key: {:?}", &self.key);
        if self.right.is_some(){
            self.right.as_ref().unwrap().inorder();
        }
    }
}

// in-order: implemented externally [by recursion]
fn inorder<T: Clone + Ord + ToString + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key: {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}