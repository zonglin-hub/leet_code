use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                return n;
            }
            set.insert(n);
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
