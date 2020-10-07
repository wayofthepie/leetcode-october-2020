use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

struct Solution;

impl Solution {
    /// The algorithm here is as follows:
    ///
    /// 1. Compute the number of moves, m, using: m = n - (k % n) - 1, where n is the length of the list
    ///    and k is the number of rotations.
    /// 2. Build a new list with the first m nodes, setting the last nodes next pointer to None.
    /// 3. Keeping the head of the last (n - m) nodes, move to the end and point the last node
    ///    to the list created in step 2. Return the head of this list.
    ///
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut head = head?;
        let length = Solution::get_length(&mut head);
        let moves = length - (k % length) - 1;
        let mut node = head.as_mut();
        let mut builder = Box::new(ListNode::new(node.val));
        let mut build_pointer = &mut builder;
        for _ in 0..moves {
            node = node.next.as_mut().unwrap();
            build_pointer.next = Some(Box::new(ListNode::new(node.val)));
            build_pointer = build_pointer.next.as_mut().unwrap();
        }
        Some(node.next.to_owned().map_or(head, |mut next| {
            Solution::build_final_list(&mut next, builder)
        }))
    }

    fn build_final_list(node: &mut Box<ListNode>, builder: Box<ListNode>) -> Box<ListNode> {
        let mut build_pointer = node.as_mut();
        while let Some(ref mut next) = build_pointer.next {
            build_pointer = next;
        }
        build_pointer.next = Some(builder);
        node.to_owned()
    }

    fn get_length(head: &mut Box<ListNode>) -> i32 {
        let mut last = head.as_mut();
        let mut length = 1;
        while let Some(ref mut next) = last.next {
            length += 1;
            last = next;
        }
        length
    }
}
