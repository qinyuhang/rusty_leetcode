/// https://leetcode.com/problems/merge-k-sorted-lists/
/// hard
use rusty_leetcode::ListNode;
use serde::{Deserialize, Serialize};

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
struct Solution;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }

        // also can use priority queue
        let mut tmp = vec![];

        for l in lists.iter() {
            let mut cur = l;
            unsafe {
                while cur.is_some() {
                    let node = cur.as_ref().unwrap();
                    let cur_pt = &**(cur.as_ref().unwrap()) as *const ListNode;
                    tmp.push(node.val);
                    cur = &(*cur_pt).next;
                }
            }
        }

        //
        tmp.sort();
        // println!("{:?}", tmp);

        let mut fake_head = Box::new(ListNode::new(i32::MIN));
        let mut cur = &mut *fake_head as *mut ListNode;
        for i in tmp {
            unsafe {
                let mut node = Box::new(ListNode::new(i));
                let cur_pt = &mut *node as *mut ListNode;
                (*cur).next = Some(node);
                cur = cur_pt;
            }
        }
        fake_head.next
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    lists: Vec<Vec<i32>>,
    result: Vec<i32>,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
    [
        {
            "lists": [],
            "result": []
        },
        {
            "lists": [[1,4,5],[1,3,4],[2,6]],
            "result": [1,1,2,3,4,4,5,6]
        },
        {
            "lists": [[]],
            "result": []
        }
    ]
    "#,
    )
    .unwrap();

    for case in test_cases {
        let mut list = vec![];
        case.lists.iter().for_each(|l| {
            let mut fake_head = Box::new(ListNode::new(-1));
            let mut cur = &mut *fake_head as *mut ListNode;
            for &i in l {
                unsafe {
                    let mut tmp = Box::new(ListNode::new(i));
                    let tmp_ptr = &mut *tmp as *mut ListNode;
                    (*cur).next = Some(tmp);
                    cur = tmp_ptr;
                }
            }
            list.push(fake_head.next);
        });

        let mut result_fake_head = Box::new(ListNode::new(-1));

        let mut cur = &mut *result_fake_head as *mut ListNode;
        for i in case.result {
            unsafe {
                let mut tmp = Box::new(ListNode::new(i));
                let tmp_ptr = &mut *tmp as *mut ListNode;
                (*cur).next = Some(tmp);
                cur = tmp_ptr;
            }
        }
        assert_eq!(Solution::merge_k_lists(list), result_fake_head.next);
    }
}

#[test]
fn main_test() {
    main();
}