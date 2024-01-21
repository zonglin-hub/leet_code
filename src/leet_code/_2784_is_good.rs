use super::Solution;

impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        let n = nums.len();
        (0..n - 1).all(|i| nums[i] == i as i32 + 1) && *nums.last().unwrap() == n as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_good() {
        assert!(!Solution::is_good(vec![2, 1, 3]));
        assert!(Solution::is_good(vec![1, 3, 3, 2]));
        assert!(Solution::is_good(vec![1, 1]));
        assert!(!Solution::is_good(vec![3, 4, 4, 1, 2, 1]));
    }
}
