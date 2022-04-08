use serde::{Deserialize, Serialize};
use serde_json;

/// https://leetcode.com/problems/median-of-two-sorted-arrays/
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let mut result = Vec::with_capacity(nums1.len() + nums2.len());

        let is_even = total_len % 2 == 0;
        let target_amount = total_len / 2;
        // println!("{}", target_amount);

        // if !is_even {
        //     target_amount += 1;
        // }

        let mut cur_n1 = 0;
        let mut cur_n2 = 0;
        // todo refine: break when get (nums1.len() + nums2.len()) / 2 + 1
        while nums1.len() > cur_n1 || nums2.len() > cur_n2 {
            let n1 = nums1.get(cur_n1).unwrap_or(&i32::MAX);
            let n2 = nums2.get(cur_n2).unwrap_or(&i32::MAX);

            let tmp = if n1 > n2 {
                cur_n2 += 1;
                n2
            } else {
                cur_n1 += 1;
                n1
            };
            result.push(tmp);
        }

        let mut r = *result[target_amount] as f64;

        if is_even {
            r = (*result[target_amount - 1] as f64 + *result[target_amount] as f64) / 2.0;
        }

        // println!("{:?}, {} {}", result, r, is_even);
        r
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    n1: Vec<i32>,
    n2: Vec<i32>,
    r: f64,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
            {
               "n1" : [1,3],
               "n2" : [2],
               "r"  : 2.0
            },
            {
               "n1" : [1,2],
               "n2" : [3,4],
               "r"  : 2.5
            }
        ]
    "#,
    )
    .unwrap();
    println!();
    for case in test_cases {
        assert_eq!(
            Solution::find_median_sorted_arrays(case.n1, case.n2),
            case.r
        )
    }
}

#[test]
fn main_test() {
    main();
}