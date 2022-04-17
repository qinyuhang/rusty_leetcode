/// https://leetcode.com/problems/binary-tree-paths/
use serde::{Deserialize, Serialize};
use serde_json;

use rusty_leetcode::{Tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        use std::collections::VecDeque;
        let mut r = vec![];

        if root.is_none() {
            return r;
        }

        let root = (
            vec![(*root.as_ref().unwrap()).borrow().val],
            Some(Rc::clone(root.as_ref().unwrap())),
        );
        let mut q = VecDeque::new();

        q.push_back(root);

        while !q.is_empty() {
            let (c, node) = q.pop_front().unwrap();

            // if left and right is none
            // it means we are reaching a leaf node
            if (node.as_ref().unwrap()).borrow().left.as_ref().is_none()
                && (node.as_ref().unwrap()).borrow().right.as_ref().is_none()
            {
                r.push(
                    c.iter()
                        .map(|&x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join("->"),
                );
            }

            // left node
            if (node.as_ref().unwrap()).borrow().left.is_some() {
                let mut arr = c.clone();
                arr.push(
                    (node.as_ref().unwrap())
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .val,
                );
                q.push_back((
                    arr,
                    Some(Rc::clone(
                        (node.as_ref().unwrap()).borrow().left.as_ref().unwrap(),
                    )),
                ));
            }
            // right node
            if (node.as_ref().unwrap()).borrow().right.as_ref().is_some() {
                let mut arr = c.clone();
                arr.push(
                    (node.as_ref().unwrap())
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .val,
                );
                q.push_back((
                    arr,
                    Some(Rc::clone(
                        (node.as_ref().unwrap()).borrow().right.as_ref().unwrap(),
                    )),
                ));
            }
        }

        r
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: String,
    o: Vec<String>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": "1,2,3,null,5",
            "o": ["1->2->5","1->3"]
        },
        {
            "i": "1",
            "o": ["1"]
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        let t = Tree::from_level_order_str(case.i);
        let mut l = std::collections::HashSet::new();
        let mut r = std::collections::HashSet::new();
        for i in Solution::binary_tree_paths(t.root) {
            l.insert(i);
        }
        for i in case.o {
            r.insert(i);
        }
        assert_eq!(l, r);
    }
}

#[test]
fn test_main() {
    main();
}
