/// https://leetcode.com/problems/permutations/
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
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    o: Vec<Vec<i32>>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": [1,2,3],
            "o": [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
        },
        {
            "i": [0,1],
            "o": [[0,1],[1,0]]
        },
        {
            "i": [1],
            "o": [[1]]
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::permute(case.i), case.o);
    }
}

#[test]
fn test_main() {
    main();
}