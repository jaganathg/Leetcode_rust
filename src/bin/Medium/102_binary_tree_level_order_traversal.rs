use std::{cell::RefCell, rc::Rc};
use std::collections::VecDeque;


struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode { 
            val: val, 
            left: None, 
            right: None 
        }
    }
}

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    if root.is_none() {
        return res;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let mut level:Vec<i32> = Vec::new();

        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            let node = node.borrow();
            level.push(node.val);

            if let Some(left) = &node.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &node.right {
                queue.push_back(right.clone());
            }
        }
        res.push(level);
    }
    res
}


fn main() {
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let mut right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);

    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = Some(Rc::new(RefCell::new(right_right)));

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    let res = level_order(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}