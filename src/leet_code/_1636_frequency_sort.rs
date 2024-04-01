use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let map = HashMap::new();
        let mut nums = nums;
        let cnt = nums.iter().fold(map, |mut m, &v| {
            *m.entry(v).or_insert(0) += 1;
            m
        });

        nums.sort_by_key(|&x| (cnt[&x], std::cmp::Reverse(x)));

        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]), vec![3, 1, 1, 2, 2, 2]);
        assert_eq!(Solution::frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
