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
        if let Some(mut next) = node.next.as_mut() {
            Solution::build_final_list(&mut next, builder)
        } else {
            Some(head)
        }
    }

    fn build_final_list(node: &mut Box<ListNode>, builder: Box<ListNode>) -> Option<Box<ListNode>> {
        let mut head = node.clone();
        let mut build_pointer = &mut head;
        while build_pointer.next.is_some() {
            build_pointer = build_pointer.next.as_mut().unwrap();
        }
        build_pointer.next = Some(builder);
        Some(head)
    }

    fn get_length(head: &mut Box<ListNode>) -> i32 {
        let mut last = head.as_mut();
        let mut length = 1;
        while last.next.is_some() {
            length += 1;
            last = last.next.as_mut().unwrap();
        }
        length
    }
}
