

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.as_mut();
    while let Some(node) = current {
        while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
            node.next = node.next.as_mut().unwrap().next.take();
        }
        current = node.next.as_mut();
    }
    head
}


fn main() {
    // Create a sample sorted linked list 1 -> 2 -> 2 -> 3 -> 3 -> 4 -> null
    let l4 = ListNode::new(4);
    let mut l3b = ListNode::new(3);
    let mut l3a = ListNode::new(3);
    let mut l2b = ListNode::new(2);
    let mut l2a = ListNode::new(2);
    let mut l1 = ListNode::new(1);

    l3b.next = Some(Box::new(l4));
    l3a.next = Some(Box::new(l3b));
    l2b.next = Some(Box::new(l3a));
    l2a.next = Some(Box::new(l2b));
    l1.next = Some(Box::new(l2a));

    let list = Some(Box::new(l1));

    // Remove duplicates
    let result = delete_duplicates(list);

    // Print the resulting linked list
    let mut current = result;
    while let Some(node) = current {
        print!("{} -> ", node.val);
        current = node.next;
    }
    println!("null");
}