use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug)]
pub enum Bst<T> {
    Leaf,
    Node(Box<TreeNode<T>>),
}

impl<T: Ord + Display> Bst<T> {
    pub fn new() -> Self {
        Bst::Leaf
    }

    pub fn insert(&mut self, elem: T) {
        match *self {
            Bst::Leaf => *self = Bst::Node(Box::new(TreeNode::new(Bst::Leaf, elem, Bst::Leaf))),
            Bst::Node(ref mut node) => match elem.cmp(&node.data) {
                Ordering::Greater => node.right.insert(elem),
                _ => node.left.insert(elem),
            },
        }
    }

    pub fn preorder(&self) {
        match *self {
            Bst::Leaf => {}
            Bst::Node(ref node) => {
                print!("{} ", node.data);
                node.left.preorder();
                node.right.preorder();
            }
        }
    }

    pub fn inorder(&self) {
        match *self {
            Bst::Leaf => {}
            Bst::Node(ref node) => {
                node.left.inorder();
                print!("{} ", node.data);
                node.right.inorder();
            }
        }
    }

    pub fn postorder(&self) {
        match *self {
            Bst::Leaf => {}
            Bst::Node(ref node) => {
                node.left.postorder();
                node.right.postorder();
                print!("{} ", node.data);
            }
        }
    }
}

#[derive(Debug)]
pub struct TreeNode<T> {
    left: Bst<T>,
    right: Bst<T>,
    data: T,
}

impl<T> TreeNode<T> {
    pub fn new(left: Bst<T>, elem: T, right: Bst<T>) -> Self {
        TreeNode {
            left: left,
            data: elem,
            right: right,
        }
    }
}
