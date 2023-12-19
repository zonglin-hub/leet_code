// use std::collections::HashMap;

// use super::Solution;

// impl Solution {
//     pub fn min_window(s: String, t: String) -> String {}
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_min_window() {
//         assert_eq!(
//             Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
//             "BANC".to_string()
//         );
//         assert_eq!(
//             Solution::min_window("a".to_string(), "a".to_string()),
//             "a".to_string()
//         );
//         assert_eq!(
//             Solution::min_window("a".to_string(), "aa".to_string()),
//             "".to_string()
//         );
//     }

//     // #[test]
//     // #[ignore = "insignificant"]
//     // fn test_entry() {
//     //     use std::collections::HashMap;

//     //     let mut letters = HashMap::new();

//     //     for ch in "a short treatise on fungi".chars() {
//     //         letters
//     //             .entry(ch)
//     //             .and_modify(|counter| *counter += 1)
//     //             .or_insert(1);
//     //     }

//     //     assert_eq!(letters[&'s'], 2);
//     //     assert_eq!(letters[&'t'], 3);
//     //     assert_eq!(letters[&'u'], 1);
//     //     assert_eq!(letters.get(&'y'), None);
//     // }

//     // #[test]
//     // #[ignore = "insignificant"]
//     // fn test_or_insert() {
//     //     use std::collections::HashMap;

//     //     let mut map: HashMap<&str, u32> = HashMap::new();
//     //     // 个表达式会查找键为"poneyland"的条目。如果找到这个条目，它就返回这个条目的引用，否则它会插入值10并返回这个新插入条目的引用。
//     //     // 然后，*= 2 操作将找到（或插入的）值乘以2。
//     //     // 所以，如果"poneyland"这个键之前不存在于map中，那么这行代码会插入值10，然后乘以2，结果为20。如果"poneyland"这个键已经存在，那么这行代码会将其值乘以2。
//     //     map.entry("poneyland").or_insert(3);
//     //     assert_eq!(map["poneyland"], 3);

//     //     *map.entry("poneyland").or_insert(10) *= 2;
//     //     assert_eq!(map["poneyland"], 6);
//     // }
// }
