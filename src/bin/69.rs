/// https://leetcode.com/problems/sqrtx/
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut r = 0;
        let mut i = 0;
        let mut tmp: i64 = i * i;

        loop {
            if tmp <= x as i64 {
                r = i;
            } else {
                break;
            }

            i += 1;
            tmp = i * i;
        }

        r as i32
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: i32,
    o: i32,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": 4,
            "o": 2
        },
        {
            "i": 8,
            "o": 2
        },
        {
            "i": 1,
            "o": 1
        },
        {
            "i": 0,
            "o": 0
        },
        {
            "i": 2147395600,
            "o": 46340
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::my_sqrt(case.i), case.o);
    }
}

#[test]
fn test_main() {
    main();
}
