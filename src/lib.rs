/**
   This is the lib used by all leetcode
   I create to help me solve the problems
*/
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Display;

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
