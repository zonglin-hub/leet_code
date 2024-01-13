use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1 = nums1.into_iter().collect::<HashSet<_>>();
        let set2 = nums2.into_iter().collect::<HashSet<_>>();

        vec![
            set1.difference(&set2).copied().collect::<Vec<_>>(),
            set2.difference(&set1).copied().collect::<Vec<_>>(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_difference() {
        assert_eq!(Solution::find_difference(vec![1, 2], vec![2, 4]), vec![vec![1], vec![4]]);
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]]
        );
    }
}
