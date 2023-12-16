// //! 根据规则将箱子分类

// use super::Solution;

// impl Solution {
//     pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
//         let a = (length | width | height) >= 10_000
//             || (length * width * height) as i64 >= 1_000_000_000;

//         let b = mass >= 100;

//         if a && b {
//             return "Both".to_string();
//         }

//         if a {
//             return "Bulky".to_string();
//         }

//         if b {
//             return "Heavy".to_string();
//         }

//         "Neither".to_string()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_categorize_box() {
//         assert_eq!(
//             Solution::categorize_box(1000, 35, 700, 300),
//             "Heavy".to_string()
//         );
//         assert_eq!(
//             Solution::categorize_box(200, 50, 800, 50),
//             "Neither".to_string()
//         );
//     }
// }
