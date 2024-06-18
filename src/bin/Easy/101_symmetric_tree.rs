
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
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


type Tree = Option<Rc<RefCell<TreeNode>>>;

fn is_symmetric(root: Tree) -> bool {
    fn is_mirror(t1: &Tree, t2: &Tree) -> bool {
        match(t1, t2) {
            (None, None) => true,
            (Some(t1), Some(t2)) => {
                let t1 = t1.borrow();
                let t2 = t2.borrow();
                t1.val == t2.val && 
                is_mirror(&t1.left, &t2.right) &&
                is_mirror(&t1.right, &t2.left)
            },
            _ => false,
        }
    }

    if let Some(root_node) = root {
        let root_ref = root_node.borrow();
        is_mirror(&root_ref.left, &root_ref.right)
    } else {
        true
    }
}

fn main() {
    // Helper function to create a new TreeNode wrapped in Rc and RefCell
    fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    // Symmetric Tree
    let root1 = new_node(1);
    root1.borrow_mut().left = Some(new_node(2));
    root1.borrow_mut().right = Some(new_node(2));
    root1.borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(new_node(3));
    root1.borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(new_node(4));
    root1.borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(new_node(4));
    root1.borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(new_node(3));

    // Non-Symmetric Tree
    let root2 = new_node(1);
    root2.borrow_mut().left = Some(new_node(2));
    root2.borrow_mut().right = Some(new_node(2));
    root2.borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(new_node(3));
    root2.borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(new_node(3));

    println!("{}", is_symmetric(Some(root1))); // Output: true
    println!("{}", is_symmetric(Some(root2))); // Output: false
}
