/// https://leetcode.com/problems/zigzag-conversion/
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    /// 切分成之字形，然后从第一行开始读

    /// 重复的pattern是 (n + n - 1)

    /// 0, 0 + (numRows * 2 - 1) * n
    /// 1, 1 + (numRows * 2 - 1) * n
    /// 2, 2 + (numRows * 2 - 1) * n
    /// ...
    /// numRows - 1, numRows - 1 + ((numRows - 1) * 2 - 1) * n
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut r = String::new();
        // once a pattern
        let pattern_count;
        let pattern_len;

        let s_char_vec = s.chars().into_iter().collect::<Vec<char>>();

        if num_rows > 2 {
            pattern_count = s_char_vec.len() as i32 / (num_rows * 2 - 2) + 1;
            pattern_len = num_rows * 2 - 2;
        } else {
            pattern_count = s_char_vec.len() as i32 / num_rows + 1;
            pattern_len = num_rows;
        }

        for i in 0..num_rows {
            // all lines total (numRows-1) line
            for j in 0..pattern_count + 1 {
                // caractor of every line it is repeat in a pattern
                if i == 0 || i == num_rows - 1 {
                    // first and last line
                    if i + pattern_len * j < s_char_vec.len() as i32 {
                        let z = String::from(s_char_vec[(i + pattern_len * j) as usize]);
                        r.push_str(&z);
                    };
                } else {
                    // other lines
                    if i + pattern_len * j < s_char_vec.len() as i32 {
                        let car_go = &String::from(s_char_vec[(i + pattern_len * j) as usize]);
                        r.push_str(car_go);
                    };
                    if (pattern_len - i) + pattern_len * j < s_char_vec.len() as i32 {
                        let car_back = &String::from(s_char_vec[((pattern_len - i) + pattern_len * j) as usize]);
                        r.push_str(car_back);
                    };
                }
            }
        }

        r
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: String,
    n: i32,
    o: String,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": "PAYPALISHIRING",
            "n": 3,
            "o": "PAHNAPLSIIGYIR"
        },
        {
            "i": "PAYPALISHIRING",
            "n": 4,
            "o": "PINALSIGYAHRPI"
        }  
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::convert(case.i, case.n), case.o)
    }
}

#[test]
fn test_main() {
    main();
}
