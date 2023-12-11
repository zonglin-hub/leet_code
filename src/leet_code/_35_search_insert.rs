//! 搜索插入位置

// use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &n) in nums.iter().enumerate() {
            if n >= target {
                return i as i32;
            }
        }

        nums.len() as i32
    }

    // pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
    //     nums.iter()
    //         .enumerate()
    //         .find_map(|(i, n)| if n >= &target { Some(i as i32) } else { None })
    //         .unwrap_or(nums.len() as i32)
    // }

    // pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    //     nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    // }

    // pub fn search_insert_3(nums: Vec<i32>, target: i32) -> i32 {
    //     if nums.is_empty() {
    //         return 0;
    //     }

    //     let mut left = 0;
    //     let mut right = nums.len() - 1;
    //     while left <= right {
    //         let mid = left + right / 2;
    //         match nums[mid].cmp(&target) {
    //             Ordering::Equal => return mid as i32,
    //             Ordering::Less => left = mid + 1,
    //             Ordering::Greater => {
    //                 if mid == 0 {
    //                     return mid as i32;
    //                 }
    //                 right = mid - 1;
    //             }
    //         }
    //     }
    //     left as i32
    // }

    // pub fn search_insert_4(nums: Vec<i32>, target: i32) -> i32 {
    //     nums.iter()
    //         .fold(0, |i, val| if val < &target { i + 1 } else { i })
    // }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
