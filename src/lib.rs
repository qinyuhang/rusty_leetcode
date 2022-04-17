/**
   This is the lib used by all leetcode
   I create to help me solve the problems
*/
use std::cell::RefCell;
use std::fmt::{Debug, Display, Write};
use std::rc::Rc;

/// TreeNode is leetcode provide data struct
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    /// this helper fn is use to be act like a BST
    #[inline]
    pub fn bst_insert(&mut self, v: i32) {
        let val = &self.val;
        if v < *val {
            if self.left.is_some() {
                (*(*(self.left.as_ref().unwrap())))
                    .borrow_mut()
                    .bst_insert(v);
                // self.left.as_mut().unwrap().insert(v);
            } else {
                self.left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            }
        } else {
            if self.right.is_some() {
                (*(*(self.right.as_ref().unwrap())))
                    .borrow_mut()
                    .bst_insert(v);
                // self.right.as_mut().unwrap().insert(v);
            } else {
                self.right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            }
        }
    }
}

/// This is a binary search tree
#[derive(Debug)]
pub struct BTree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

impl BTree {
    pub fn new() -> Self {
        BTree { root: None }
    }
    pub fn insert(&mut self, val: i32) {
        if let Some(ref mut node) = self.root {
            (*(*node).as_ref().borrow_mut()).bst_insert(val);
        } else {
            self.root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
}

/// This is a normol tree
#[derive(Debug)]
pub struct Tree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}

// These codes are from quest 297
impl Tree {
    pub fn from_level_order_str(data: String) -> Self {
        use std::collections::VecDeque;
        let mut tree = Tree { root: None };

        let mut chars = data
            .split(',')
            .into_iter()
            .filter(|&c| c != "")
            .collect::<VecDeque<&str>>();

        if chars.len() == 0 {
            return tree;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(
            chars.pop_front().unwrap().parse().unwrap(),
        )));

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();

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

        tree.root = Some(root);

        tree
    }
    pub fn to_level_order_str(&self) -> String {
        let root = Some(Rc::clone(self.root.as_ref().unwrap()));

        let mut res: Vec<String> = vec![];
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(root);

        // level travers
        while let Some(entity) = queue.pop_front() {
            if let Some(ref node) = entity {
                let bo_node = Rc::clone(node);

                res.push((*bo_node).borrow().val.to_string());

                if (*bo_node).borrow().left.as_ref().is_some() {
                    queue.push_back(Some(Rc::clone(&(*bo_node).borrow().left.as_ref().unwrap())));
                } else {
                    queue.push_back(None);
                }

                if (*bo_node).borrow().right.as_ref().is_some() {
                    queue.push_back(Some(Rc::clone(
                        &(*bo_node).borrow().right.as_ref().unwrap(),
                    )));
                } else {
                    queue.push_back(None);
                }
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
}

/// Linked List
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(format!("LinkedList: [{}", self.val).as_str());
        if self.next.is_some() {
            let mut cur = self.next.as_ref();
            while cur.is_some() {
                output.push_str(format!(", {}", cur.unwrap().val).as_str());
                cur = cur.unwrap().next.as_ref();
            }
        }
        output.push_str("]");
        f.write_str(&output)
    }
}

#[test]
fn test_tree_serialize_deserialize() {
    let case = "1,2,3,4,5,6,7".to_string();
    let t = Tree::from_level_order_str(case.clone());
    assert_eq!(t.to_level_order_str(), case);
}
