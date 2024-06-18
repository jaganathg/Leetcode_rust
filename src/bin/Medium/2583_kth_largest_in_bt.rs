use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;


struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> TreeNode {
        TreeNode { 
            val: val, 
            left: None, 
            right: None 
        }
    }
}


fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    let mut res_list: Vec<i32> = Vec::new();
    if root.is_none() {
        return -1;
    }
    let mut q = VecDeque::new();
    q.push_back(root);

    

    while !q.is_empty() {
        let level_size = q.len();
        let mut sum = 0;

        for _ in 0..level_size {
            if let Some(Some(node)) = q.pop_front() {
                let node = node.borrow();
                sum += node.val;

                if node.left.is_some() {
                    q.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    q.push_back(node.right.clone());
                }
            }
        }
        res_list.push(sum);
    }
    if k > res_list.len() as i32 {
        return -1;
    }

    res_list.sort();
    res_list.reverse();

    res_list[k as usize - 1] as i64

}

fn main() {
    // Example usage:
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let k = 1;
    let result = kth_largest(root, k);
    println!("The {}-th largest level sum is {}", k, result);
}


pub fn _kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, sums: &mut Vec<i64>) {
        if let Some(r) = root {
            if depth == sums.len() {
                sums.push(0);
            }
            let r = r.borrow();
            sums[depth] += r.val as i64;
            dfs(r.left.clone(), depth + 1, sums);
            dfs(r.right.clone(), depth + 1, sums);
        }
    }
    let mut sums = vec![];
    dfs(root, 0, &mut sums);
    sums.sort();
    sums.reverse();
    let k = k as usize - 1;
    if sums.len() > k {
        sums[k]
    } else {
        -1
    }       
}