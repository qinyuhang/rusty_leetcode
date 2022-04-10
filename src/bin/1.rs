/// https://leetcode.com/problems/two-sum/
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::with_capacity(2);
        let mut hash = HashMap::<i32, usize>::new();
        for (i, x) in nums.iter().enumerate() {
            if hash.contains_key(&x) {
                result.push(
                    *hash.get(&x).unwrap() as i32
                );
                result.push(i as i32);
            } else {
                hash.insert(target - x, i);
            }
        }
        return result;
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    nums: Vec<i32>,
    target: i32,
    o: Vec<i32>,
}

fn main() {

    let test_cases: Vec<TestCase> = serde_json::from_str(r#"
    [
        {
            "nums": [2,7,11,15],
            "target": 9,
            "o": [0,1]
        },
        {
            "nums": [3,2,4],
            "target": 6,
            "o": [1,2]
        },
        {
            "nums": [3,3],
            "target": 6,
            "o": [0,1]
        }
    ]"#).unwrap();

    for case in test_cases {
        assert_eq!(Solution::two_sum(case.nums, case.target), case.o);
    }
}

#[test]
fn test_main() {
    main();
}