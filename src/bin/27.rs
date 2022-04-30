/// https://leetcode.com/problems/remove-element/
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut r = nums.len();
        let nl = nums.len();
        for (i, &v) in nums.clone().iter().enumerate() {
            if v == val {
                nums.remove(i - (nl - r));
                r -= 1;
            }
        }
        r as i32
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    v: i32,
    o: Vec<i32>,
    r: i32,
}

fn main() {
    let test_case: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": [3,2,2,3],
            "v": 3,
            "o": [2,2],
            "r": 2
        }
    ]"#,
    )
    .unwrap();

    for case in test_case {
        let mut ci = case.i.clone();
        let r = Solution::remove_element(&mut ci, case.v);
        assert_eq!(r, case.r);
        assert_eq!(ci, case.o);
    }
}

#[test]
fn test_remove_element() {
    main();
}
