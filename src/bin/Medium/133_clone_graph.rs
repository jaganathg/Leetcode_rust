
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
         Rc::new(RefCell::new(Node {
            val,
            neighbors: vec![],
         }))
    }
}

fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut visited: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();

    if let Some(n) = node.clone() {
        return Some(dfs(n, &mut visited));
    }
    None
}

fn dfs(node: Rc<RefCell<Node>>, visited: &mut HashMap<i32, Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
    let val = node.borrow().val;

    if let Some(n) = visited.get(&val) {
        // return n.clone();
        panic!("Done complete round and my value is {:?}", n);
    }

    let copy = Node::new(val);
    visited.insert(val, copy.clone());
    println!("my value is {:?}", &copy);

    for neighbor in &node.borrow().neighbors{
        let neighbor_copy = dfs(neighbor.clone(), visited);
        copy.borrow_mut().neighbors.push(neighbor_copy);
    }
    copy
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    let node4 = Node::new(4);

    node1.borrow_mut().neighbors.push(node2.clone());
    node1.borrow_mut().neighbors.push(node4.clone());
    node2.borrow_mut().neighbors.push(node1.clone());
    node2.borrow_mut().neighbors.push(node3.clone());
    node3.borrow_mut().neighbors.push(node2.clone());
    node3.borrow_mut().neighbors.push(node4.clone());
    node4.borrow_mut().neighbors.push(node1.clone());
    node4.borrow_mut().neighbors.push(node3.clone());

    let cloned_graph = clone_graph(Some(node1));

    println!("{:?}", cloned_graph);
}