// Level-order traversal visits the nodes layer by layer.
// A queue data structure is required for level-order traversal

use std::collections::VecDeque;
use std::fmt::Debug;
use crate::BinaryTree;

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    fn levelorder(&self) {
        let size = self.size();
        let mut q = VecDeque::new();
        
        // enqueue the root node
        let _r = q.push_back(Box::new(self.clone()));
        while !q.is_empty() {
            // dequeue the first node, and output its value
            let front = q.pop_front().unwrap();
            println!("key: {:?}", front.get_key());
            
            // enqueue the left/right child node
            match front.get_left() { 
                Some(left) => {
                    let _r = q.push_back(left);
                },
                None => {},
            }
            
            match front.get_right() { 
                Some(right) => {
                    let _r = q.push_back(right);
                },
                None => {},
            }
        }
    }
}