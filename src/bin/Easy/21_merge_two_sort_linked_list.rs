
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn merge_two_sorted_linked_list(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
   match(l1, l2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => {
            if l1.val < l2.val {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_sorted_linked_list(l1.next, Some(l2))
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_sorted_linked_list(Some(l1), l2.next)
                }))
            }
        }
    }
} 

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    }));
    let result = merge_two_sorted_linked_list(l1, l2);
    println!("{:?}", result);
}
