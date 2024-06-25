use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

    let mut total = 0;

    convert(root.clone(), &mut total);
    root

}

fn convert(node: Option<Rc<RefCell<TreeNode>>>, total: &mut i32) {
    if let Some(n) = node {
        let right = n.borrow().right.clone();
        convert(right, total);
        let mut n_borrowed = n.borrow_mut();
        *total += n_borrowed.val;
        n_borrowed.val = *total;
        let left = n_borrowed.left.clone();
        drop(n_borrowed);
        convert(left, total);
    }
    

}

fn main() {
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.left.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let result = bst_to_gst(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}