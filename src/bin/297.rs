use rusty_leetcode::TreeNode;
/// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
///
///
use serde::{Deserialize, Serialize};
use serde_json;

use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        return Codec {};
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res: Vec<String> = vec![];
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(root);

        // level travers
        while let Some(entity) = queue.pop_front() {
            if let Some(ref node) = entity {
                res.push(node.borrow().val.to_string());
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            } else {
                res.push(String::from("null"));
            }
        }

        // remove the null from the end
        while res.last() == Some(&"null".to_string()) {
            res.pop();
        }

        res.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        let mut chars = data
            .split(',')
            .into_iter()
            .filter(|&c| c != "")
            .collect::<VecDeque<&str>>();

        if chars.len() == 0 {
            return None;
        }

        let mut root = Rc::new(RefCell::new(TreeNode::new(
            chars.pop_front().unwrap().parse().unwrap(),
        )));

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        while !queue.is_empty() {
            let mut cur = queue.pop_front().unwrap();

            let left_ch = chars.pop_front();
            let right_ch = chars.pop_front();
            if left_ch.is_some() {
                let ch = left_ch.unwrap();
                if ch != "null" {
                    let node = Rc::new(RefCell::new(TreeNode::new(ch.parse().unwrap())));

                    queue.push_back(Rc::clone(&node));
                    (*cur).borrow_mut().left = Some(node);
                };
            }
            if right_ch.is_some() {
                let ch = right_ch.unwrap();
                if ch != "null" {
                    let node = Rc::new(RefCell::new(TreeNode::new(ch.parse().unwrap())));

                    queue.push_back(Rc::clone(&node));
                    (*cur).borrow_mut().right = Some(node);
                }
            }
        }

        Some(root)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: String,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": "1,2,3,null,null,4,5"
        },
        {
            "i": ""
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        let c = Codec::new();
        assert_eq!(case.i, c.serialize(c.deserialize(case.i.clone())));
    }
}

#[test]
fn test_main() {
    main();
}
