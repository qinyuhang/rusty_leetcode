/// https://leetcode.com/problems/merge-two-sorted-lists/
use rusty_leetcode::ListNode;
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut fake_head = Box::new(ListNode::new(-1));
        let mut cur = &mut *fake_head as *mut ListNode;
        let mut tmp;
        let mut t1 = list1;
        let mut t2 = list2;

        while t1.is_some() || t2.is_some() {
            if t1.is_some() && t2.is_some() {
                let val1 = (t1.as_ref().unwrap()).val;
                let val2 = (t2.as_ref().unwrap()).val;

                if val1 < val2 {
                    tmp = Some(Box::new(ListNode::new(val1)));
                    t1 = t1.unwrap().next;
                } else {
                    tmp = Some(Box::new(ListNode::new(val2)));
                    t2 = t2.unwrap().next;
                }
            } else {
                if t1.is_some() {
                    let val1 = (t1.as_ref().unwrap()).val;

                    tmp = Some(Box::new(ListNode::new(val1)));
                    t1 = t1.unwrap().next;
                } else {
                    let val2 = (t2.as_ref().unwrap()).val;

                    tmp = Some(Box::new(ListNode::new(val2)));
                    t2 = t2.unwrap().next;
                }
            }
            unsafe {
                let mut tmp = tmp.unwrap();
                let tmp_raw_pt = &mut *tmp as *mut ListNode;
                (*cur).next = Some(tmp);
                cur = tmp_raw_pt;
                // cur = &mut (*tmp_raw_pt) as *mut Box<ListNode>;
            }
        }
        fake_head.next
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    j: Vec<i32>,
    o: Vec<i32>,
}
fn main() {

    let test_cases: Vec<TestCase> = serde_json::from_str(r#"
        [
            {
                "i": [],
                "j": [],
                "o": []
            },
            {
                "i": [1, 2, 4],
                "j": [1, 3, 4],
                "o": [1, 1, 2, 3, 4, 4]
            }
        ]
    "#).unwrap();

    for case in test_cases {
        let mut t1 = Box::new(ListNode::new(-1));
        let mut t2 = Box::new(ListNode::new(-1));
        let mut r = Box::new(ListNode::new(-1));
    
        let mut cur = &mut (*t1) as *mut ListNode;
        for i in case.i {
            unsafe {
                let mut tmp = Box::new(ListNode::new(i));
                let tmp_pt = &mut *tmp as *mut ListNode;
                (*cur).next = Some(tmp);
                cur = tmp_pt;
            }
        }
    
        let mut cur = &mut (*t2) as *mut ListNode;
        for i in case.j {
            // t1.
            unsafe {
                let mut tmp = Box::new(ListNode::new(i));
                let tmp_pt = &mut *tmp as *mut ListNode;
                (*cur).next = Some(tmp);
                cur = tmp_pt;
            }
        }
        
        let mut cur = &mut (*r) as *mut ListNode;
        for i in case.o {
            // t1.
            unsafe {
                let mut tmp = Box::new(ListNode::new(i));
                let tmp_pt = &mut *tmp as *mut ListNode;
                (*cur).next = Some(tmp);
                cur = tmp_pt;
            }
        }
        // println!("{:?} {:?}", t1, t2);
        // [1,2,4]
        // [1,3,4]
        // println!("{:?}", Solution::merge_two_lists(t1.next, t2.next));
        assert_eq!(Solution::merge_two_lists(t1.next, t2.next), r.next);
    }
    
}

#[test]
fn main_test() {
    main();
}