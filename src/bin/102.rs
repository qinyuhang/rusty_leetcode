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
        node_stack.push_back(Rc::clone(&root.as_ref().unwrap()));
        let mut node;
        let mut cur_level = 0;
        while node_stack.len() != 0 {
            node = node_stack.pop_front();
            let mut next_level = cur_level;
            if node_stack.len() == 0 {
                next_level = cur_level + 1;
            }
            let tmp = node;
            if tmp.as_ref().unwrap().borrow().left.is_some() {
                node_stack.push_back(Rc::clone(
                    &tmp.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                ));
            }
            if result.get(cur_level).is_none() {
                result.push(vec![]);
            }
            result[cur_level].push(tmp.as_ref().unwrap().borrow().val);
            if tmp.as_ref().unwrap().borrow().right.is_some() {
                node_stack.push_back(Rc::clone(
                    &tmp.as_ref().unwrap().borrow().right.as_ref().unwrap(),
                ));
            }
            // 时机不对
            cur_level = next_level;
        }
        result
    }
}

#[derive(Serialize, Deserialize)]
struct TestCase {
    case: Vec<i32>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "case": [3, 9, 20, 15, 7]
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
        println!("{:?}", Solution::level_order(bt.root));
    }
}

#[test]
fn main_test() {
    main();
}
