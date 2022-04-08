/// https://leetcode.com/problems/squares-of-a-sorted-array/
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut r = nums.iter().map(|n| n * n).collect::<Vec<_>>();
        r.sort();

        r
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    s: Vec<i32>,
    r: Vec<i32>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "s": [-4,-1,0,3,10],
            "r": [0,1,9,16,100]
        },
        {
            "s": [-7,-3,2,3,11],
            "r": [4,9,9,49,121]
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::sorted_squares(case.s), case.r);
    }
}
#[test]
fn main_test() {
    main();
}
