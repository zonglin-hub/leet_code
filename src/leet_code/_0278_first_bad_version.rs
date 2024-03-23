// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
// use super::Solution;

// impl Solution {
//     pub fn first_bad_version(&self, n: i32) -> i32 {
//         let mut left = 1;
//         let mut right = n;

//         while left < right {
//             let mid = left + (right - left) / 2;

//             if self.isBadVersion(mid) {
//                 
// right = mid;
//             } else {
//                left = mid + 1;
//             }
//         }
//         left
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_first_bad_version() {
//         assert_eq!(Solution.first_bad_version(5), 4);
//         assert_eq!(Solution.first_bad_version(1), 1);
//     }
// }
