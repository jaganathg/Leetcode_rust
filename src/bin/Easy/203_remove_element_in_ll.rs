#[derive(Debug, Clone, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}


fn remove_element_in_ll(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    
    while let Some(ref node) = head {
        if node.val == val {
            head = node.next.clone();
        } else {
            break;
        }
    }

    let mut curr = &mut head;
        while let Some(ref mut node) = curr {
            while let Some(ref next_node) = node.next {
                if next_node.val == val {
                    node.next = next_node.next.clone();
                } else {
                    break;
                }
            }
            curr = &mut node.next;
        }
        head
}


fn leetCode_solutions(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut prev = &mut dummy;

    while let Some(mut node) = head.take() {
        head = node.next.take();

        if node.val != val {
            *prev = Some(node);
            prev = &mut prev.as_mut().unwrap().next;
        }
    }
    dummy
}

fn main() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut curr = &mut head;
    for i in 2..6 {
        let node = Some(Box::new(ListNode::new(i)));
        match curr {
            Some(ref mut n) => {
                n.next = node;
                curr = &mut n.next;
            }
            None => break,
        }
    }

    let mut curr = &head;
    while let Some(ref node) = curr {
        println!("{}", node.val);
        curr = &node.next;
    }

    let new_head = leetCode_solutions(head, 3);
    let mut curr = &new_head;
    while let Some(ref node) = curr {
        println!("{}", node.val);
        curr = &node.next;
    }
}