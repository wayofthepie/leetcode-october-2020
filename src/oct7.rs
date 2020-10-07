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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head.as_ref()?;
        if k == 0 {
            return head;
        }
        let length = {
            let mut last = head.as_mut().unwrap();
            let mut length = 1;
            while last.next.is_some() {
                length += 1;
                last = last.next.as_mut().unwrap();
            }
            length
        };
        let mut moves = length - (k % length) - 1;
        let mut node = head.as_mut().unwrap();
        let mut new_head = Box::new(ListNode::new(node.val));
        let mut build_pointer = &mut new_head;
        while moves > 0 {
            node = node.next.as_mut().unwrap();
            let new = Box::new(ListNode::new(node.as_ref().val));
            build_pointer.next = Some(new);
            build_pointer = build_pointer.next.as_mut().unwrap();
            moves -= 1;
        }
        if node.next.is_some() {
            node = node.next.as_mut().unwrap();
            let mut real_head = node.clone();
            let mut build_pointer = &mut real_head;
            while build_pointer.next.is_some() {
                build_pointer = build_pointer.next.as_mut().unwrap();
            }
            build_pointer.next = Some(new_head);
            Some(real_head)
        } else {
            head
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, Solution};

    #[test]
    fn example() {
        let head = Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode { next: None, val: 4 })),
                    val: 3,
                })),
                val: 2,
            })),
            val: 1,
        });
        let answer = Solution::rotate_right(Some(head), 41);
        println!("{:#?}", answer);
        assert!(false);
    }
}
