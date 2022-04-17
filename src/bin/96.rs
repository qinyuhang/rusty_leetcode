/// https://leetcode.com/problems/unique-binary-search-trees
use rusty_leetcode::BTree;
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn backtrack(nums: &Vec<i32>, track: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // 当track被塞入了与原始素材一样多的时候就表示走到叶节点了
        if nums.len() == track.len() {
            result.push(track.clone());
            return;
        }

        for i in nums.iter() {
            if track.contains(i) {
                continue;
            }

            // 前序遍历
            track.push(*i);

            Solution::backtrack(nums, track, result);

            // 后序遍历
            track.pop();
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut r = vec![];

        let mut track = vec![];

        Solution::backtrack(&nums, &mut track, &mut r);

        r
    }
    pub fn num_trees(n: i32) -> i32 {
        let arrs = Solution::permute((1..=n).collect());
        // let set = std::collections::HashSet::new();
        let mut r = vec![];
        for i in arrs {
            let mut t = BTree::new();
            for j in i {
                t.insert(j);
            }
            if !r.contains(&t) {
                r.push(t);
            }
        }
        r.len() as i32
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    n: i32,
    o: i32,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "n": 3,
            "o": 5
        },
        {
            "n": 1,
            "o": 1
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::num_trees(case.n), case.o);
    }
}

#[test]
fn test_main() {
    main();
}
