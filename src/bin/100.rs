use rusty_leetcode::{BTree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut result = false;

        if p.is_none() && q.is_none() {
            result = true;
        }
        if p.is_some() && q.is_some() {
            if let Some(p_v) = p {
                if let Some(q_v) = q {
                    if (*p_v).borrow().val != (*q_v).borrow().val {
                        result = false;
                    } else {
                        let mut left_same = false;
                        let mut right_same = false;
                        if (*p_v).borrow().left.is_none() && (*q_v).borrow().left.is_none() {
                            left_same = true;
                        } else if (*p_v).borrow().left.is_some() && (*q_v).borrow().left.is_some() {
                            left_same = Solution::is_same_tree(
                                Some(Rc::clone(&(*p_v).borrow().left.as_ref().unwrap())),
                                Some(Rc::clone(&(*q_v).borrow().left.as_ref().unwrap())),
                            );
                        }
                        if (*p_v).borrow().right.is_none() && (*q_v).borrow().right.is_none() {
                            right_same = true;
                        } else if (*p_v).borrow().right.is_some() && (*q_v).borrow().right.is_some()
                        {
                            right_same = Solution::is_same_tree(
                                Some(Rc::clone(&(*p_v).borrow().right.as_ref().unwrap())),
                                Some(Rc::clone(&(*q_v).borrow().right.as_ref().unwrap())),
                            );
                        }
                        result = left_same && right_same;
                    }
                } else {
                    result = false;
                }
            };
        }

        result
    }
}

fn main() {
    let mut tr1 = BTree::new();
    let mut tr2 = BTree::new();
    let mut tr3 = BTree::new();

    for (idx, val) in [1, 2, 3].iter().enumerate() {
        tr1.insert(*val);
        tr2.insert(*val);
        tr3.insert(idx as i32);
    }

    assert_eq!(Solution::is_same_tree(tr1.root, tr2.root.clone()), true);
    assert_eq!(Solution::is_same_tree(tr2.root, tr3.root), false);
}

#[test]
fn main_test() {
    main();
}