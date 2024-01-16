// use super::Solution;

// impl Solution {
//     pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
//         const N: i32 = 23;
//         const M: i32 = 401;
//         const MOD: i64 = 1000_000_007;

//         let d;
//         let num;
//         let min_sum;
//         let max_sum;

//         fn dfs(num: String, i: i32, j: i32, limit: bool, max_sum: i32, min_sum: i32) -> i32 {
//             if j > max_sum {
//                 return 0;
//             }
//             if i == -1 {
//                 return if j >= min_sum { 1 } else { 0 };
//             }
//         }

//         // fn get(num: String) -> i32 {
//         //     num = num.split("").rev().to_owned();

//         // }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_color_the_array() {
//         assert_eq!(Solution::count("1".to_owned(), "12".to_owned(), 1, 8), 11);
//         assert_eq!(Solution::count("1".to_owned(), "5".to_owned(), 1, 5), 5);
//     }
// }
