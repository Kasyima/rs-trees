// Trees
// The key is how to define tree nodes.

mod parse_tree;
mod pre_order_traversal;
mod post_order_traversal;
mod in_order_traversal;
mod level_order_traversal;

use std::cmp::{max, Ordering::*};
use std::fmt::{Debug, Display};

// Binary tree child node link
type Link<T> = Option<Box<BinaryTree<T>>>;

// Binary tree definition
#[derive(Debug, Clone, PartialEq)]
struct BinaryTree<T> {
    key: T,         // data storage
    left: Link<T>,  // left child node address storage
    right: Link<T>, // right child node address storage
}

// The struct has a key that stores data and left and right pointers that stores the addresses of the left and the right child nodes.

impl<T: Clone + Ord + ToString + Debug> BinaryTree<T> {
    fn new(key: T) -> Self {
        Self {
            key: key,
            left: None,
            right: None,
        }
    }

    // *****INSERTION******
    // There are two possible cases to consider:
    // When the node has no child nodes --- the new    node can be directly inserted as its child.
    // When the node has child nodes --- the new node's child nodes must first be attached to the appropriate positions in the tree before the new node itself can be added as a child of the original node.

    // New child node is added as the left child node of the root node
    fn insert_left_node(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    // New child is added as the right child node of the root node
    fn insert_right_node(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    // calculate total node number
    fn size(&self) -> usize {
        self.calc_size(0)
    }

    fn calc_size(&self, mut size: usize) -> usize {
        size += 1;

        if !self.left.is_none() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }
        if !self.right.is_none() {
            size = self.right.as_ref().unwrap().calc_size(size);
        }

        size
    }

    // calculate the total leaf nodes number
    fn leaf_size(&self) -> usize {
        // both left and right nodes are none which means current node is a leaf node
        if self.left.is_none() && self.right.is_none() {
            return 1;
        }

        // calculate the total leaf nodes of both left and right subtrees
        let left_leaf = match &self.left {
            Some(left) => left.leaf_size(),
            None => 0,
        };
        let right_leaf = match &self.right {
            Some(right) => right.leaf_size(),
            None => 0,
        };

        // leaf nodes = leaf nodes(left) + leaf nodes(right)
        left_leaf + right_leaf
    }

    // calculate non-leaf nodes, it is actually very easy to calculate
    fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    // calculate the depth of a tree
    fn depth(&self) -> usize {
        let mut left_depth = 1;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        // return the max depth
        max(left_depth, right_depth)
    }

    // get left subtree
    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    // get the key of the node
    fn get_key(&self) -> T {
        self.key.clone()
    }

    // set the key of the node
    fn set_key(&mut self, key: T) {
        self.key = key;
    }

    //
    fn min(&self) -> Option<&T> {
        let mut node = self;
        while let Some(left) = &node.left {
            node = left;
        }
        Some(&node.key)
    }

    fn max(&self) -> Option<&T> {
        let mut node = self;
        while let Some(right) = &node.right {
            node = right;
        }
        Some(&node.key)
    }

    // determine whether a key is in the tree
    fn contains(&self, key: &T) -> bool {
        match &self.key.cmp(key) {
            Equal => true,
            Less => match &self.right {
                Some(right) => right.contains(key),
                None => false,
            },
            Greater => match &self.left {
                Some(left) => left.contains(key),
                None => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
