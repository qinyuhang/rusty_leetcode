/// https://leetcode.com/problems/house-robber/
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
        let n_l = nums.len();
        let mut dp = Vec::with_capacity(n_l);

        dp.push(nums[0]);
        dp.push(nums[0].max(nums[1]));
        for i in 2..n_l {
            dp.push(dp[i - 1].max(nums[i] + dp[i - 2]));
        }

        dp[n_l - 1]
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
            "i": [1,2,3,1],
            "o": 4
        },
        {
            "i": [2,7,9,3,1],
            "o": 12
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::rob(case.i), case.o);
    }
}

#[test]
fn test_house_robber() {
    main();
}
