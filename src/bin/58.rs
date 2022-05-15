/// https://leetcode.com/problems/length-of-last-word/
///
///
use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .split(' ')
            .last()
            .unwrap_or("")
            .chars()
            .into_iter()
            .collect::<Vec<_>>()
            .len() as i32
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    i: String,
    o: i32,
}

fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(
        r#"[
        {
            "i": "hello world",
            "o": 5
        },
        {
            "i": "   fly me   to   the moon  ",
            "o": 4
        },
        {
            "i": "luffy is still joyboy",
            "o": 6
        }
    ]"#,
    )
    .unwrap();

    for case in test_cases {
        assert_eq!(Solution::length_of_last_word(case.i), case.o);
    }
}

#[test]
fn test_main() {
    main();
}
