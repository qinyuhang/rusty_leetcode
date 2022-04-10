// /**
//  * @param {string} s
//  * @return {number}
//  */
// var lengthOfLongestSubstring = function(s) {
//     var hashMap = {};
//     var result = 0;
//     var j = 0
//     for (var i = 0; i < s.length; i++) {
//         if (hashMap[s[i]] == undefined) {
//             hashMap[s[i]] = i;
//             result = Math.max(result, i + 1 - j);
//         } else {
//             if (j > hashMap[s[i]]) {
//                 // do nothing
//                 result = Math.max(result, i + 1 -j)
//                 hashMap[s[i]] = i
//             } else {
//                 j = hashMap[s[i]] + 1;
//                 result = Math.max(result, i + 1 -j)
//                 hashMap[s[i]] = i
//             }
//         }
//     }
//     return result;
// };

use serde::{Deserialize, Serialize};
use serde_json;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut dp = HashMap::new();
        // j is the index of a substring's begin char
        let mut j = 0;
        let mut r = 0;

        for (idx, ch) in s.chars().into_iter().enumerate() {
            if !dp.contains_key(&ch) {
                dp.insert(ch, idx);
                r = r.max(idx + 1 - j);
            } else {
                let &dp_i = dp.get(&ch).unwrap();

                if j > dp_i {
                    r = r.max(idx + 1 - j);
                    dp.insert(ch, idx);
                } else {
                    j = dp_i + 1;
                    r = r.max(idx + 1 - j);
                    dp.insert(ch, idx);
                }
            }
        }

        r as i32
        
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    s: String,
    o: i32,
}


fn main() {
    let test_cases: Vec<TestCase> = serde_json::from_str(r#"[
        {
            "s": "abcabcbb",
            "o": 3
        }
    ]"#).unwrap();

    for case in test_cases {
        assert_eq!(
            Solution::length_of_longest_substring(case.s),
            case.o
        )
    }
}

#[test]
fn test_main() {
    main();
}