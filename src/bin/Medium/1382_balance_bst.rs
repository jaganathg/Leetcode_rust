use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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

fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    
    let mut nodes = Vec::new();
    inorder(&root, &mut nodes);
    build_bst(&nodes, 0, nodes.len())
    
}

fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        inorder(&node.left, nodes);
        nodes.push(node.val);
        inorder(&node.right, nodes);
    }
}

fn build_bst(nodes: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if start == end {
        return None;
    }

    let mid = (start + end) / 2;
    
    Some(Rc::new(RefCell::new(TreeNode {
        val: nodes[mid],
        left: build_bst(nodes, start, mid),
        right: build_bst(nodes, mid + 1, end),
    })))
}

fn main() {
    // Example 1: [1, null, 2, null, 3, null, 4, null, null]
    let mut root = TreeNode::new(1);
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    
    let balanced_root = balance_bst(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", balanced_root); // Implementing a way to print the tree is left as an exercise
}