use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Solution::build_node(val);
        if root == None {
            return node;
        }
        Some(Solution::insert_rec(&mut root.unwrap(), val))
    }

    fn insert_rec(root: &mut Rc<RefCell<TreeNode>>, val: i32) -> Rc<RefCell<TreeNode>> {
        if val < root.as_ref().borrow().val {
            let mut root = root.borrow_mut();
            if let Some(left) = root.left.as_mut() {
                Solution::insert_rec(left, val);
            } else {
                root.left = Solution::build_node(val);
            }
        } else {
            let mut root = root.borrow_mut();
            if let Some(right) = root.right.as_mut() {
                Solution::insert_rec(right, val);
            } else {
                root.right = Solution::build_node(val);
            }
        }
        root.to_owned()
    }

    fn build_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::{Solution, TreeNode};

    #[test]
    fn example() {
        let node = None;
        println!("{:#?}", Solution::insert_into_bst(node, 9));
        assert!(false);
    }
}
