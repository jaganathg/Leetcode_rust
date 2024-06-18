
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode { 
            val,
            next: None }
    }
}


fn _reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut curr = head;
    let mut prev:Option<Box<ListNode>> = None;
    
    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        // Create a linked list 1 -> 2 -> 3
        let mut list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: None,
                })),
            })),
        }));

        // Reverse the list
        list = _reverse_list(list);

        // Check that the list is now 3 -> 2 -> 1
        assert_eq!(list.as_ref().unwrap().val, 3);
        list = list.unwrap().next;
        assert_eq!(list.as_ref().unwrap().val, 2);
        list = list.unwrap().next;
        assert_eq!(list.as_ref().unwrap().val, 1);
    }
}


fn main() {

}