/// https://leetcode.com/problems/merge-two-sorted-lists/
use rusty_leetcode::ListNode;

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
fn main() {
    let mut t1 = Box::new(ListNode::new(-1));
    let mut t2 = Box::new(ListNode::new(-1));

    let mut cur = &mut (*t1) as *mut ListNode;
    for i in [1, 2, 4] {
        unsafe {
            let mut tmp = Box::new(ListNode::new(i));
            let tmp_pt = &mut *tmp as *mut ListNode;
            (*cur).next = Some(tmp);
            cur = tmp_pt;
        }
    }

    let mut cur = &mut (*t2) as *mut ListNode;
    for i in [1, 3, 4] {
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
    println!("{:?}", Solution::merge_two_lists(t1.next, t2.next));
}

#[test]
fn main_test() {
    main();
}