use super::Solution;

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1]
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_running_sum() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(Solution::running_sum(vec![1, 1, 1, 1]), vec![1, 2, 3, 4]);
        assert_eq!(Solution::running_sum(vec![3, 1, 2, 10]), vec![3, 4, 6, 16]);
    }
}
