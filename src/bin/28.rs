/// https://leetcode.com/problems/implement-strstr/
/// 
/// 
/// 
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut r = -1;

        let h_str = haystack.as_str();
        let n_str = needle.as_str();

        for i in 0..h_str.len() {
            if h_str[i..].starts_with(n_str) {
                r = i as i32;
                break;
            }
        }
        r
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    h: String,
    n: String,
    r: i32,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(r#"[
        {
            "h": "hello",
            "n": "ll",
            "r": 2
        }
    ]"#).unwrap();

    for case in test_cases {
        assert_eq!(Solution::str_str(case.h, case.n), case.r);
    }
}

#[test]
fn test_strstr() {
    main();
}