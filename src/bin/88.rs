/// https://leetcode.com/problems/merge-sorted-array/
/// modify nums1 in place
/// 
use serde::{Deserialize, Serialize};
use serde_json;
struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut result = [
            nums1
                .iter()
                .take(m as usize)
                .map(|&x| x)
                .collect::<Vec<i32>>(),
            nums2
                .iter()
                .take(n as usize)
                .map(|&x| x)
                .collect::<Vec<i32>>(),
        ]
        .concat();
        result.sort();
        nums1.clear();
        for i in result {
            nums1.push(i);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: Vec<i32>,
    m: i32,
    j: Vec<i32>,
    n: i32,
    o: Vec<i32>,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(r#"
    [
        {
            "i": [1,2,3,0,0,0],
            "m": 3,
            "j": [2,5,6],
            "n": 3,
            "o": [1,2,2,3,5,6]
        }
    ]
    "#).unwrap();

    for case in test_cases {
        let mut i = case.i.clone();
        let mut j = case.j.clone();
        Solution::merge(&mut i, case.m, &mut j, case.n);
        assert_ne!(case.i, i);
        assert_eq!(i, case.o);
    }
}

#[test]
fn main_test() {
    main();
}
