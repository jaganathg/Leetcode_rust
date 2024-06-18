
use std::rc::Rc;
use std::cell::RefCell;


pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    if root.is_none() {
        return res;
    }

    let mut stack = vec![];
    let mut curr = root;

    while curr.is_some() || !stack.is_empty() {
        while let Some(node) = curr {
            stack.push(node.clone());
            curr = node.borrow().left.clone();
        }
        curr = stack.pop();
        if let Some(node) = curr.as_ref() {
            let right_child = node.borrow().right.clone();
            res.push(node.borrow().val);
            curr = right_child;
        }
    }
    res
}

fn main() {
    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    let right_left = TreeNode::new(3);

    right.left = Some(Rc::new(RefCell::new(right_left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    let result = inorder_traversal(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}