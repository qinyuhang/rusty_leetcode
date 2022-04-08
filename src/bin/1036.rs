/// https://leetcode.com/problems/escape-a-large-maze/
/// https://leetcode.com/problems/escape-a-large-maze/discuss/1481290/python-bfs-with-improvement
use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::{HashSet, VecDeque};
// use std::
use std::rc::Rc;
struct Solution;
impl Solution {
    #[allow(unused)]
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        // make the blocked to a map
        let mut block_set = HashSet::with_capacity(blocked.len());

        for b in blocked {
            block_set.insert(Rc::new(b));
            // block_set.insert(format!("{},{}", b[0], b[1]));
        }
        // just do dfs
        let mut q = VecDeque::new();
        let mut visited = HashSet::<Rc<Vec<i32>>>::new();
        let start = Rc::new(source);
        q.push_back(Rc::clone(&start));
        visited.insert(Rc::clone(&start));

        let mut steps = 1;

        // TODO 50 steps
        while q.len() != 0 && steps < 50 {
            steps += 1;
            let size = q.len();
            for i in 0..size {
                let cur_node = q.pop_front().unwrap();
                let right_node = Rc::new(vec![cur_node[0] + 1, cur_node[1]]);
                let down_node = Rc::new(vec![cur_node[0], cur_node[1] + 1]);
                // push right, down node
                if !block_set.contains(&right_node) && !visited.contains(&right_node) {
                    q.push_back(Rc::clone(&right_node));
                    visited.insert(Rc::clone(&right_node));
                }
                if !block_set.contains(&down_node) && !visited.contains(&down_node) {
                    q.push_back(Rc::clone(&down_node));
                    visited.insert(Rc::clone(&down_node));
                }
                if cur_node[0] == target[0] && cur_node[1] == target[1] {
                    return true;
                }
            }
            // how to stop?
        }
        false
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    start: Vec<i32>,
    end: Vec<i32>,
    block: Vec<Vec<i32>>,
    result: bool,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
        [
            {
                "start": [0, 0],
                "end": [1, 1],
                "block": [[0,1], [1,0]],
                "result": false
            },
            {
                "start": [0, 0],
                "end": [999999,999999],
                "block": [[0,1]],
                "result": true
            },
            {
                "start": [0, 0],
                "end": [999999,999999],
                "block": [],
                "result": true
            }

        ]
        "#,
    )
    .unwrap();

    println!("{:?}", test_cases);

    for case in test_cases {
        assert_eq!(
            Solution::is_escape_possible(case.block, case.start, case.end),
            case.result
        );
    }
}
