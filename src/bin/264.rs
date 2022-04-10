use serde::{Deserialize, Serialize};
use serde_json;
// 首先起始 dp = [1,2,3,5]
struct Solution;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![1];

        let mut c2 = 0;
        let mut c3 = 0;
        let mut c5 = 0;

        for i in 1..n {
            let r = (dp[c2] * 2).min(dp[c3] * 3).min(dp[c5] * 5);
            dp.push(r);

            if dp[c2] * 2 == dp[i as usize] {
                c2 += 1
            }
            if dp[c3] * 3 == dp[i as usize] {
                c3 += 1
            }
            if dp[c5] * 5 == dp[i as usize] {
                c5 += 1
            }
        }

        dp[(n - 1) as usize] as i32

        // for z in 0..(1 + n / 2) {
        //     for y in 0..(1 + n / 2) {
        //         for x in 0..(1 + n) {
        //             // println!(
        //             //     "x: {}, y: {}, z: {}, result: {}",
        //             //     x as u32,
        //             //     y as u32,
        //             //     z as u32,
        //             //     i32::pow(2, x as u32) * i32::pow(3, y as u32) * i32::pow(5, z as u32)
        //             // );
        //             dp.push(
        //                 i128::pow(2, x as u32) * i128::pow(3, y as u32) * i128::pow(5, z as u32),
        //             );
        //         }
        //     }
        // }
        // dp.sort();
        // // println!("dp: {:?}", dp);
        // // tmp = (2 ** x) * (3 ** y) * (5 ** z)

        // dp[(n - 1) as usize] as i32
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    n: i32,
    r: i32,
}
fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "n": 10,
            "r": 12
        },
        {
            "n": 1,
            "r": 1
        },
        {
            "n": 14,
            "r": 20
        },
        {
            "n": 2,
            "r": 2
        },
        {
            "n": 31,
            "r": 81
        },
        {
            "n": 57,
            "r": 324
        },
        {
            "n": 1352,
            "r": 402653184
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::nth_ugly_number(case.n), case.r);
        println!("{} th ugly number is {}", case.n, case.r);
    }
}

#[test]
fn test_main() {
    main();
}
