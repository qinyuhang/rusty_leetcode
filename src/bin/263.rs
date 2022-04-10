use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut tmp = n;
        let mut result = true;
        if tmp == 0 {
            return false;
        }
        while tmp != 1 {
            if tmp % 2 == 0 {
                tmp = tmp / 2;
            }
            else if tmp % 3 == 0 {
                tmp = tmp / 3;
            }
            else if tmp % 5 == 0 {
                tmp = tmp / 5;
            }
            else {
                result = false;
                break;
            }
        }
        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    n: i32,
    r: bool,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
    [
        {
            "n": 6,
            "r": true
        },
        {
            "n": 1,
            "r": true
        },
        {
            "n": 14,
            "r": false
        }
    ]
    "#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::is_ugly(case.n), case.r);
    }
}
#[test]
fn main_test() {
    main();
}
