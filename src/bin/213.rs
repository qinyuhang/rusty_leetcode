/// https://leetcode.com/problems/house-robber-ii/
///
///
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let n_r = nums.iter().rev().map(|&x| x).collect::<Vec<i32>>();
        let n_l = nums.len();
        let mut dp = vec![];
        let mut dp_back = vec![];

        // 正向
        dp.push(nums[0]);
        dp.push(nums[0].max(nums[1]));
        // 反向
        dp_back.push(n_r[0]);
        dp_back.push(n_r[0].max(n_r[1]));
        for i in 2..n_l {
            // 正向
            dp.push(dp[i - 1].max(nums[i] + dp[i - 2]));
            // 反向
            dp_back.push(dp_back[i - 1].max(n_r[i] + dp_back[i - 2]));
        }

        // 因为是环形，所以要再减一
        dp[n_l - 2].max(dp_back[n_l - 2])
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    o: i32,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": [2,3,2],
            "o": 3
        },
        {
            "i": [1,2,3,1],
            "o": 4
        },
        {
            "i": [0],
            "o": 0
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::rob(case.i), case.o);
    }
}

#[test]
fn test_robber_ii() {
    main();
}
