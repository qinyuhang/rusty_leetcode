use rusty_leetcode::{BTree, TreeNode};

struct Solution;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut node_stack = VecDeque::new();
        if root.is_none() {
            return result;
        }

        result.push(vec![]);
        node_stack.push_back((0, Rc::clone(&root.as_ref().unwrap())));

        let mut node;

        while node_stack.len() != 0 {
            node = node_stack.pop_front();

            if let Some((cur_level, tmp)) = node {
                if tmp.borrow().left.is_some() {
                    node_stack.push_back((
                        cur_level + 1,
                        Rc::clone(&tmp.borrow().left.as_ref().unwrap()),
                    ));
                }

                if result.get(cur_level).is_none() {
                    result.push(vec![]);
                }
                result[cur_level].push(tmp.borrow().val);

                if tmp.borrow().right.is_some() {
                    node_stack.push_back((
                        cur_level + 1,
                        Rc::clone(&tmp.borrow().right.as_ref().unwrap()),
                    ));
                }
            }
        }
        result
    }
}

#[derive(Serialize, Deserialize)]
struct TestCase {
    case: Vec<i32>,
    o: Vec<Vec<i32>>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "case": [9, 3, 20, 15, 7],
            "o": [[9], [3, 20], [7, 15]]
        }
    ]"#,
    )
    .unwrap();

    for i in test_cases {
        let mut bt = BTree::new();
        for j in i.case {
            bt.insert(j);
        }
        // println!("{:?}", bt);
        // println!("{:?}", Solution::level_order(bt.root));

        assert_eq!(Solution::level_order(bt.root), i.o);
    }
}

#[test]
fn main_test() {
    main();
}
