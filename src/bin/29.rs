/// https://leetcode.com/problems/divide-two-integers/
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let r = dividend as i64 / divisor as i64;
        if r >= i32::MAX as i64 {
            return i32::MAX;
        };
        if r <= i32::MIN as i64 {
            return i32::MIN;
        }
        r as i32
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    d: i32,
    or: i32,
    r: i32,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "d": 10,
            "or": 3,
            "r": 3
        },
        {
            "d": 7,
            "or": -3,
            "r": -2
        },
        {
            "d": -2147483648,
            "or": -1,
            "r": 2147483647
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::divide(case.d, case.or), case.r);
    }
}

#[test]
fn test_divide() {
    main();
}
