// use super::Solution;

// use std::collections::{HashSet, VecDeque};

// impl Solution {
//     #[allow(clippy::needless_range_loop)]
//     pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
//         let mut word_set = word_list.into_iter().collect::<HashSet<String>>();
//         if !word_set.contains(&end_word) {
//             return 0;
//         }

//         let mut word_queue = VecDeque::new();
//         word_queue.push_back((begin_word, 1));

//         while let Some((mut src, idx)) = word_queue.pop_front() {
//             if end_word.eq(&src) {
//                 return idx;
//             }

//             let u8arr = unsafe { &mut *(src.as_bytes_mut() as *mut [u8]) };

//             for i in 0..u8arr.len() {
//                 let origin_c = u8arr[i];

//                 for c in b'a'..=b'z' {
//                     if origin_c != c {
//                         u8arr[i] = c;
//                         if let Some(new_src) = word_set.take(&src) {
//                             word_queue.push_back((new_src, idx + 1));
//                         }
//                     }
//                 }

//                 u8arr[i] = origin_c;
//             }
//         }

//         0
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_ladder_length() {
//         assert_eq!(
//             Solution::ladder_length(
//                 "hit".to_string(),
//                 "cog".to_string(),
//                 vec![
//                     "hot".to_string(),
//                     "dot".to_string(),
//                     "dog".to_string(),
//                     "lot".to_string(),
//                     "log".to_string(),
//                     "cog".to_string()
//                 ]
//             ),
//             5
//         );
//         assert_eq!(
//             Solution::ladder_length(
//                 "hit".to_string(),
//                 "cog".to_string(),
//                 vec![
//                     "hot".to_string(),
//                     "dot".to_string(),
//                     "dog".to_string(),
//                     "lot".to_string(),
//                     "log".to_string()
//                 ]
//             ),
//             0
//         );
//     }
// }
