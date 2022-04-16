/// https://leetcode.com/problems/reverse-integer/
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut r: i64 = 0;

        let sign: i64 = if x < 0 { -1 } else { 1 };

        let mut tmp = (x as i64) * sign;

        while tmp > 0 {
            r = r * 10 + (tmp % 10) as i64;

            if sign * r > i32::MAX as i64 || sign * r < i32::MIN as i64 {
                return 0;
            }

            tmp = tmp / 10;
        }

        (r * sign) as i32
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
            "i": 123,
            "o": 321
        },
        {
            "i": -123,
            "o": -321
        },
        {
            "i": 120,
            "o": 21
        },
        {
            "i": 0,
            "o": 0
        },
        {
            "i": 2147483647,
            "o": 0
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::reverse(case.i), case.o)
    }
}

#[test]
fn test_main() {
    main();
}
