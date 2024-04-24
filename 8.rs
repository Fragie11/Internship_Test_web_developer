// Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::cmp;

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
        match root {
            Some(node) => {
                let left_depth = Self::max_depth(&node.left);
                let right_depth = Self::max_depth(&node.right);
                cmp::max(left_depth, right_depth) + 1
            }
            None => 0,
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    let left = Some(Box::new(TreeNode::new(9)));
    let right = Some(Box::new(TreeNode::new(20)));
    right.as_ref().unwrap().left = Some(Box::new(TreeNode::new(15)));
    right.as_ref().unwrap().right = Some(Box::new(TreeNode::new(7)));
    root.left = left;
    root.right = right;
    println!("Max depth of the binary tree is: {}", TreeNode::max_depth(&Some(Box::new(root))));
}
