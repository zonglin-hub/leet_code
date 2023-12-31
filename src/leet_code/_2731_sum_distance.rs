// //! 移动机器人

// use super::Solution;

// impl Solution {
//     pub fn sum_distance(mut nums: Vec<i32>, s: String, d: i32) -> i32 {
//         nums.iter_mut().enumerate().for_each(|(idx, x)| {
//             match s.chars().collect::<Vec<char>>()[idx] {
//                 'R' => *x += d,
//                 'L' => *x -= d,
//                 _ => panic!(),
//             }
//         });
//         nums.sort();

//         let md = 1_000_000_007;
//         (nums
//             .iter()
//             .enumerate()
//             .map(|(idx, x)| (*x) as i64 * idx as i64 - (nums.len() - idx - 1) as i64 * (*x) as i64)
//             .reduce(|acc, e| (acc + e) % md)
//             .unwrap()
//             % md) as i32
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_color_the_array() {
//         assert_eq!(
//             Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3),
//             8
//         );
//         assert_eq!(Solution::sum_distance(vec![1, 0], "RL".to_string(), 2), 5);
//     }
// }
