use rusty_leetcode::ListNode;
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        let mut l1 = l1;
        let mut l2 = l2;

        let mut fake_head = Box::new(ListNode::new(-1));
        let mut build_cur = &mut *fake_head as *mut ListNode;

        let mut jin_wei = 0;

        while l1.is_some() || l2.is_some() || jin_wei != 0 {
            let t1 = l1.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
            let t2 = l2.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;


            let mut node = Box::new(ListNode::new((jin_wei + t1 + t2) % 10));
            let node_pt = &mut *node as *mut ListNode;

            jin_wei = (t1 + t2 + jin_wei) / 10;
            unsafe {
                (*build_cur).next = Some(node);
            }
            build_cur = node_pt;

            if l1.is_some() {
                l1 = l1.unwrap().next;
            }
            if l2.is_some() {
                l2 = l2.unwrap().next;
            }
        }

        fake_head.next
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    l1: Vec<i32>,
    l2: Vec<i32>,
    o: Vec<i32>,
}
fn main() {

    let test_cases: Vec<TestCase> = serde_json::from_str(r#"[

        {
            "l1": [2,4,3],
            "l2": [5,6,4],
            "o": [7,0,8]
        },
        {
            "l1": [0],
            "l2": [0],
            "o": [0]
        },
        {
            "l1": [9,9,9,9,9,9,9],
            "l2": [9,9,9,9],
            "o": [8,9,9,9,0,0,0,1]
        }
    ]"#).unwrap();


    for case in test_cases {
        let mut l1 = Box::new(ListNode::new(-1));
        let mut l1_pt = &mut *l1 as *mut ListNode;
        
        let mut l2 = Box::new(ListNode::new(-2));
        let mut l2_pt = &mut *l2 as *mut ListNode;
        
        let mut o = Box::new(ListNode::new(-2));
        let mut o_pt = &mut *o as *mut ListNode;

        for l1_cur in case.l1 {
            unsafe {
                let mut node = Box::new(ListNode::new(l1_cur));
                let node_pt = &mut *node as *mut ListNode;
                (*l1_pt).next = Some(node);
                l1_pt = node_pt;
            }
        }
        
        for l2_cur in case.l2 {
            unsafe {
                let mut node = Box::new(ListNode::new(l2_cur));
                let node_pt = &mut *node as *mut ListNode;
                (*l2_pt).next = Some(node);
                l2_pt = node_pt;
            }
        }
        
        for o_cur in case.o {
            unsafe {
                let mut node = Box::new(ListNode::new(o_cur));
                let node_pt = &mut *node as *mut ListNode;
                (*o_pt).next = Some(node);
                o_pt = node_pt;
            }
        }

        assert_eq!(Solution::add_two_numbers(l1.next, l2.next), o.next);


    }
}

#[test]
fn test_main() {
    main();
}