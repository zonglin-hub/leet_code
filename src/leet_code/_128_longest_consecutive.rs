use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_set = HashSet::new();

        for num in nums {
            num_set.insert(num);
        }

        let mut longest_streak = 0;

        for num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let mut current_num = *num;
                let mut current_streak = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                longest_streak = longest_streak.max(current_streak);
            }
        }
        longest_streak
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
