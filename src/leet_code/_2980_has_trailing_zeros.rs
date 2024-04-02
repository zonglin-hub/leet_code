use super::Solution;

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        nums.iter().filter(|&x| x % 2 == 0).count() >= 2
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::has_trailing_zeros(vec![1, 2, 3, 4, 5]));
        assert!(Solution::has_trailing_zeros(vec![2, 4, 8, 16]));
        assert!(!Solution::has_trailing_zeros(vec![1, 3, 5, 7, 9]));
    }
}
