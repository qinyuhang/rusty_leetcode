use rusty_leetcode::{BTree, TreeNode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;


/// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        if nums.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: nums[0],
                left: None,
                right: None,
            })));
        }
        let mid_idx = nums.len() / 2;
        let node_val = nums[mid_idx];
        let mut node = TreeNode {
            val: node_val,
            left: None,
            right: None,
        };
        node.left = Solution::sorted_array_to_bst(nums[0..mid_idx].to_vec());
        node.right = Solution::sorted_array_to_bst(nums[mid_idx + 1..].to_vec());
        Some(Rc::new(RefCell::new(node)))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    case: Vec<i32>,
    o: Vec<i32>,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(r#"
    [
        {
            "case": [-10,-3,0,5,9],
            "o": [0,-3,9,-10,5]
        }
    ]"#).unwrap();

    for case in test_cases {
        let mut tree = BTree::new();
        for i in case.o {
            tree.insert(i);
        }
        assert_eq!(Solution::sorted_array_to_bst(case.case), tree.root);
    }
}

#[test]
fn main_test() {
    main();
}