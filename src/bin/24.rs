/// 24. Swap Nodes in Pairs
/// https://leetcode.com/problems/swap-nodes-in-pairs/
use rusty_leetcode::ListNode;
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn revers_list_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut walk_cur = head.as_ref();

        let mut fake_head = Box::new(ListNode::new(-1));
        let mut build_cur_pt = &mut *fake_head as *mut ListNode;

        while walk_cur.is_some() {
            let cur_pt = &**walk_cur.unwrap() as *const ListNode;

            unsafe {
                let val = (*cur_pt).val;
                let mut node = Box::new(ListNode::new(val));
                let mut node_pt = &mut *node as *mut ListNode;
                // let tmp hold build_head.next
                // take is used to a Option val
                let tmp_box = (*build_cur_pt).next.take();
                // set build_head.next = new Node
                (*build_cur_pt).next = Some(node);
                // set new Node.next = tmp
                (*node_pt).next = tmp_box; //Some(*tmp.unwrap());

                // walk next node
                walk_cur = walk_cur.as_ref().unwrap().next.as_ref();
            }
        }
        fake_head.next
    }
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // call revers_list_node 2 as a group,
        // break the list 2 a group
        // so first build fake 2 a group
        let mut fake_head = Box::new(ListNode::new(-1));
        let mut build_cur_pt = &mut *fake_head as *mut ListNode;
        let mut walk_cur = head.as_ref();
        let swap_group = 2;

        while walk_cur.is_some() {            
            unsafe {
                let mut group_fake_head = build_cur_pt;
                let mut group_count = 0;
                for _ in 0..swap_group {
                    if walk_cur.is_some() {
                        group_count += 1;
                        let mut node = Box::new(ListNode::new(walk_cur.unwrap().val));
                        let node_pt = &mut *node as *mut ListNode;
                        (*group_fake_head).next = Some(node);
                        // build a swap_group len ListNode
                        // then call revers
                        group_fake_head = node_pt;
                        walk_cur = walk_cur.unwrap().next.as_ref();
                    }
                }
                // do swap
                // todo use a cur
                
                // (*build_cur_pt).next = Solution::revers_list_node((*build_cur_pt).next.take());
                

                // move build_cur_pt to point the end of the list
                // next code work fine in my mac but not in leetcode,
                // so I have to change to just copy all the ListNode
                // while (*build_cur_pt).next.is_some() {
                //     let mut next_node = (*build_cur_pt).next.take().unwrap();
                //     // let next_node = Box::new(ListNode::new());
                //     let next_node_pt = &mut *next_node as *mut ListNode;
                //     build_cur_pt = next_node_pt;
                // }

                let mut result =
                    if group_count == swap_group {
                        Solution::revers_list_node((*build_cur_pt).next.take())
                    } else {
                        (*build_cur_pt).next.take()
                    };
                
                while result.is_some() {
                    let val = result.as_ref().unwrap().val;
                    let mut new_node = Box::new(ListNode::new(val));
                    let new_node_pt = &mut *new_node as *mut ListNode;
                    (*build_cur_pt).next = Some(new_node);
                    build_cur_pt = new_node_pt;
                    result = result.unwrap().next.take();
                }
            }   
        }
        fake_head.next
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    o: Vec<i32>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
    [
        {
            "i": [1,2,3,4],
            "o": [2,1,4,3]
        },
        {
            "i": [],
            "o": []
        },
        {
            "i": [1],
            "o": [1]
        }
    ]
    "#,
    )
    .unwrap();

    for case in test_cases {
        let mut fake_i_head = Box::new(ListNode::new(-1));
        let mut fake_o_head = Box::new(ListNode::new(-1));
        let mut i_cur_pt = &mut *fake_i_head as *mut ListNode;
        let mut o_cur_pt = &mut *fake_o_head as *mut ListNode;

        for i in case.i {
            unsafe {
                let mut node = Box::new(ListNode::new(i));
                let node_pt = &mut *node as *mut ListNode;
                (*i_cur_pt).next = Some(node);
                i_cur_pt = node_pt;
            }
        }

        for i in case.o {
            unsafe {
                let mut node = Box::new(ListNode::new(i));
                let node_pt = &mut *node as *mut ListNode;
                (*o_cur_pt).next = Some(node);
                o_cur_pt = node_pt;
            }
        }

        // println!("{:?}", fake_i_head.next);
        // println!("{:?}", Solution::revers_list_node(fake_i_head.next));

        assert_eq!(Solution::swap_pairs(fake_i_head.next), fake_o_head.next);
    }
}

#[test]
fn main_test() {
    main();
}
