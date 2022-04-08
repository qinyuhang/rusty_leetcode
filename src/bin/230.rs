use rusty_leetcode::{BTree, TreeNode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut r = vec![];
        if let Some(node) = root {
            if node.borrow().left.is_some() {
                r.append(&mut Solution::dfs(Some(Rc::clone(
                    node.borrow().left.as_ref().unwrap(),
                ))));
            }
            r.push(node.borrow().val);
            if node.borrow().right.is_some() {
                r.append(&mut Solution::dfs(Some(Rc::clone(
                    node.borrow().right.as_ref().unwrap(),
                ))));
            }
        }
        r
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let r = Solution::dfs(root);
        r[(k - 1) as usize]
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    case: Vec<i32>,
    k: i32,
    result: i32,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
    [
        {
            "case": [3, 1, 4, 2],
            "k": 1,
            "result": 1
        },
        {
            "case": [5, 3, 6, 2, 4, 1],
            "k": 3,
            "result": 3
        }
    ]
    "#,
    )
    .unwrap();

    for case in test_cases {
        let mut tree = BTree::new();
        for j in case.case {
            tree.insert(j);
        }
        assert_eq!(Solution::kth_smallest(tree.root, case.k), case.result);
    }
}

#[test]
fn main_test() {
    main();
}
