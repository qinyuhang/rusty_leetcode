use rusty_leetcode::{BTree, TreeNode};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut node_stack = vec![];

        let mut cur_node = root;
        let mut inorder = vec![];

        // loop iter b-tree
        while cur_node != None || node_stack.len() != 0 {
            let tmp = cur_node;
            // preorder_walk

            cur_node = None;

            node_stack.push(Rc::clone(&tmp.as_ref().unwrap()));
            if tmp.as_ref().unwrap().borrow().left.is_some() {
                cur_node = Some(Rc::clone(&tmp.unwrap().borrow().left.as_ref().unwrap()));
            }
            while cur_node == None && node_stack.len() != 0 {
                cur_node = node_stack.pop();
                if cur_node.is_some() {
                    let tmp = cur_node;
                    cur_node = None;

                    // inorder iter
                    inorder.push(tmp.as_ref().unwrap().borrow().val);

                    if tmp.as_ref().unwrap().borrow().right.is_some() {
                        cur_node = Some(Rc::clone(&tmp.unwrap().borrow().right.as_ref().unwrap()))
                    }
                }
                // node = stack.pop();
                // node = node.right;
            }
        }

        println!("{:?}", inorder);

        // let mut compare_mark = i32::MIN;
        let mut result = vec![];
        for i in inorder[0..inorder.len() - 1].iter().enumerate() {
            result.push(*i.1 < inorder[i.0 + 1]);
        }
        result.iter().fold(true, |a, b| a && *b)
        // inorder.iter().fold(true, |a, b| {
        //     let r = *b > compare_mark || (compare_mark == i32::MIN && *b == i32::MIN);
        //     compare_mark = *b;
        //     a && r
        // })
    }
}

#[derive(Deserialize, Serialize)]
struct TestCase {
    case: Vec<i32>,
    result: bool,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
        [
            {
                "case": [-2147483648],
                "result": true
            },
            {
                "case": [2,1,3],
                "result": true
            }
        ]
        "#,
    )
    .unwrap();
    // let test_case = ; //[2, 1, 3];

    for i in test_cases {
        let mut tree = BTree { root: None };
        for j in i.case {
            tree.insert(j);
            // let mut tn = TreeNode::new(test_case[0]);

            // for i in test_case[1..test_case.len()].iter() {
            //     tn.bst_insert(*i);
            // }

            // println!("{:?}", tn);
        }
        println!("{:?}", tree);
        // println!(
        //     "{}",
        //     Solution::is_valid_bst(Some(Rc::new(RefCell::new(tn))))
        // );
        // println!("{}", Solution::is_valid_bst(tree.root));
        assert_eq!(Solution::is_valid_bst(tree.root), i.result);
    }

    // let mut b_t = MBTree::Node(Some(Box::new(MBTreeNode::new(test_case[0]))));

    // if let MBTree::Node(Some(ref mut root)) = b_t {

    //     // for i in 20..50 {
    //     //     root.insert(i);
    //     // }

    //     // for i in 2..15 {
    //     //     root.insert(i);
    //     // }

    //     // for i in vec![9,3,5,0,50,70,8,7,11,34,100] {
    //     //     root.insert(i);
    //     // }
    //     for i in test_case[1..test_case.len()].iter() {
    //         root.insert(*i);
    //     }

    //     println!("{:?}", root);

    //     println!("start root.preorder_walk");
    //     root.inorder_walk(|x| println!("{}", x));
    //     println!("finish root.preorder_walk");

    //     // println!("{}", Solution::is_valid_bst())
    // };
}

#[test]
fn main_test() {
    main();
}
