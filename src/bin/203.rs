use rusty_leetcode::ListNode;
/// https://leetcode.com/problems/remove-linked-list-elements/
///
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut list = None;
        let mut cur = &mut list;
        let mut old_list = head;

        while let Some(mut node) = old_list.take() {
            old_list = node.next.take();
            if node.val != val {
                cur = &mut cur.insert(node).next;
            }
        }

        list
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    val: i32,
    o: Vec<i32>,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": [1,2,6,3,4,5,6],
            "val": 6,
            "o": [1,2,3,4,5]
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(
            Solution::remove_elements(ListNode::from_iter(case.i), case.val),
            ListNode::from_iter(case.o)
        )
    }
}

#[test]
fn test_main() {
    main();
}
