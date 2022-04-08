use serde::{Deserialize, Serialize};
/// https://leetcode.com/problems/shortest-distance-to-a-character/
/// 一种解法
/// 使用两个游标
/// 设置 整个 vec 为 [0; s.char().len()];
/// 从左边开始遍历
/// mark = 0
/// cur = 0 的时候，为 [0,0,0]
/// cur = 1, [1,0,0]
/// cur = 2, [2,1,0]
/// 一直到找到第一个 char
/// 从这以后，mark = cur, cur = cur+1, 每个元素的值为 min(to mark, to cur)
/// mark = 3, cur = 4, [2,1, min(to mark, to cur)]
use serde_json;

struct Solution;
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result = vec![0i32; s.chars().into_iter().collect::<Vec<char>>().len()];

        let target_vec = s
            .chars()
            .into_iter()
            .enumerate()
            .filter(|item| item.1 == c)
            .collect::<Vec<(usize, char)>>();

        if target_vec.len() == 0 {
            return result;
        }

        let left_mark_idx = 0;
        let mut right_mark_idx = 1;
        if target_vec.len() < 2 {
            right_mark_idx = 0;
        }
        let mut left_mark = target_vec[left_mark_idx];
        let mut right_mark = target_vec[right_mark_idx];

        for (idx, _ch) in s.chars().enumerate() {
            if idx < left_mark.0 {
                result[idx] = (left_mark.0 - idx) as i32;
            }
            if idx > left_mark.0 {
                if idx <= right_mark.0 {
                    result[idx] = ((idx - left_mark.0).min(right_mark.0 - idx)) as i32;
                } else {
                    result[idx] = (idx - left_mark.0) as i32;
                }
            }
            if idx == right_mark.0 {
                // move marks
                left_mark = right_mark;
                if right_mark_idx + 1 < target_vec.len() {
                    right_mark_idx += 1;
                    right_mark = target_vec[right_mark_idx];
                }
            }
        }

        result
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct TestCase {
    s: String,
    c: char,
    r: Vec<i32>,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"
    [
        {
            "s": "loveleetcode",
            "c": "e",
            "r": [3,2,1,0,1,0,0,1,2,2,1,0]
        },
        {
            "s": "a",
            "c": "a",
            "r": [0]
        },
        {
            "s": "",
            "c": "a",
            "r": []
        },
        {
            "s": "aaba",
            "c": "b",
            "r": [2,1,0,1]
        }
    ]
    "#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::shortest_to_char(case.s, case.c), case.r);
    }
}
#[test]
fn main_test() {
    main();
}
