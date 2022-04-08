#[allow(unused_imports)]
use rusty_leetcode::{BTree, TreeNode};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

struct Solution;
impl Solution {
    #[allow(unused)]
    fn compare(
        node_left: Option<Rc<RefCell<TreeNode>>>,
        node_right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if node_left.is_none() && node_right.is_none() {
            return true;
        }

        if node_left.as_ref().is_some()
            && node_right.as_ref().is_some()
            && node_left.as_ref().unwrap().deref().borrow().val
                == node_right.as_ref().unwrap().deref().borrow().val
        {
            let node_left_left_child = if node_left
                .as_ref()
                .unwrap()
                .deref()
                .borrow()
                .left
                .as_ref()
                .is_some()
            {
                Some(Rc::clone(
                    &node_left
                        .as_ref()
                        .unwrap()
                        .deref()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap(),
                ))
            } else {
                None
            };

            let node_left_right_child = if node_left
                .as_ref()
                .unwrap()
                .deref()
                .borrow()
                .right
                .as_ref()
                .is_some()
            {
                Some(Rc::clone(
                    &node_left
                        .as_ref()
                        .unwrap()
                        .deref()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap(),
                ))
            } else {
                None
            };
            let node_right_left_child = if node_right
                .as_ref()
                .unwrap()
                .deref()
                .borrow()
                .left
                .as_ref()
                .is_some()
            {
                Some(Rc::clone(
                    &node_right
                        .as_ref()
                        .unwrap()
                        .deref()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap(),
                ))
            } else {
                None
            };

            let node_right_right_child = if node_right
                .as_ref()
                .unwrap()
                .deref()
                .borrow()
                .right
                .as_ref()
                .is_some()
            {
                Some(Rc::clone(
                    &node_right
                        .as_ref()
                        .unwrap()
                        .deref()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap(),
                ))
            } else {
                None
            };
            return Solution::compare(node_left_right_child, node_right_left_child)
                && Solution::compare(node_left_left_child, node_right_right_child);
        } else {
            return false;
        }
    }

    #[allow(unused)]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let left_node = if root
            .as_ref()
            .unwrap()
            .deref()
            .borrow()
            .left
            .as_ref()
            .is_some()
        {
            Some(Rc::clone(
                &root
                    .as_ref()
                    .unwrap()
                    .deref()
                    .borrow()
                    .left
                    .as_ref()
                    .unwrap(),
            ))
        } else {
            None
        };

        let right_node = if root
            .as_ref()
            .unwrap()
            .deref()
            .borrow()
            .right
            .as_ref()
            .is_some()
        {
            Some(Rc::clone(
                root.as_ref()
                    .unwrap()
                    .deref()
                    .borrow()
                    .right
                    .as_ref()
                    .unwrap(),
            ))
        } else {
            None
        };

        Solution::compare(left_node, right_node)
    }
}

fn main() {}
