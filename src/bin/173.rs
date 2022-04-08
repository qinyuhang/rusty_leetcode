use rusty_leetcode::{BTree, TreeNode};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

struct BSTIterator {
    cur: Cell<usize>,
    vec: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        // do a inorder iter over the BST
        let mut result = vec![];
        let mut node = root;
        let mut stack = vec![];
        while node != None || stack.len() != 0 {
            let tmp = node;
            node = None;

            stack.push(Rc::clone(&tmp.as_ref().unwrap()));

            if tmp.as_ref().unwrap().borrow().left.is_some() {
                node = Some(Rc::clone(&tmp.unwrap().borrow().left.as_ref().unwrap()));
            }

            while node == None && stack.len() != 0 {
                node = stack.pop();
                if node.is_some() {
                    let tmp = node;
                    node = None;

                    result.push(tmp.as_ref().unwrap().borrow().val);

                    if tmp.as_ref().unwrap().borrow().right.is_some() {
                        node = Some(Rc::clone(&tmp.unwrap().borrow().right.as_ref().unwrap()));
                    }
                }
            }
        }
        BSTIterator {
            cur: Cell::new(0),
            vec: result,
        }
    }

    fn next(&self) -> i32 {
        let r = self.vec[self.cur.get()];
        self.cur.set(self.cur.get() + 1);
        r
    }

    fn has_next(&self) -> bool {
        self.cur.get() < (self.vec).len()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
// struct Solution;
// impl Solution {}

fn main() {
    // let tree = BTr
    let mut tree = BTree::new();
    for j in [1, 2, 3] {
        tree.insert(j);
    }
    let bti = BSTIterator::new(tree.root);
    assert_eq!(bti.has_next(), true);
    assert_eq!(bti.next(), 1);
    assert_eq!(bti.next(), 2);
    assert_eq!(bti.next(), 3);
    assert_eq!(bti.has_next(), false);
}

#[test]
fn main_test() {
    main();
}
