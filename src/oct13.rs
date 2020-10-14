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

use std::{cmp::Reverse, collections::BinaryHeap};
struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut to_sort = Vec::new();
        let mut pointer = &mut head;
        while let Some(ref mut node) = pointer {
            to_sort.push(node.val);
            pointer = &mut node.next;
        }
        to_sort.sort();
        to_sort.reverse();
        if let Some(init) = to_sort.pop() {
            let mut new_head = Some(Box::new(ListNode::new(init)));
            let mut pointer = &mut new_head;
            while let Some(val) = to_sort.pop() {
                pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
                pointer = &mut pointer.as_mut().unwrap().next
            }
            return new_head;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, Solution};

    #[test]
    fn should_return_none_if_head_is_none() {
        let head = None;
        assert_eq!(None, Solution::sort_list(head));
    }

    #[test]
    fn example1() {
        let mut n1 = ListNode::new(4);
        let mut n2 = ListNode::new(2);
        let n3 = ListNode::new(7);
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));
        let head = Some(Box::new(n1));
        let mut n1 = ListNode::new(2);
        let mut n2 = ListNode::new(4);
        let n3 = ListNode::new(7);
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));
        let mut expected = Some(Box::new(n1));

        let mut actual_pointer = &mut Solution::sort_list(head);
        let mut expected_pointer = &mut expected;
        while let Some(ref mut node) = actual_pointer {
            assert_eq!(node.val, expected_pointer.as_ref().unwrap().val);
            expected_pointer = &mut expected_pointer.as_mut().unwrap().next;
            actual_pointer = &mut node.as_mut().next;
        }
    }
}
