use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1 = nums1.into_iter().collect::<HashSet<i32>>();
        let nums2 = nums2.into_iter().collect::<HashSet<i32>>();
        nums1.intersection(&nums2).map(|&x| x).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_restore_string() {
        assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        // assert_eq!(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
    }
}
