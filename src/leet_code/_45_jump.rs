use super::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut step = 0;
        let mut max = 0;
        let mut end = 0;

        // for i in 0..nums.len() - 1 { 等同
        for (i, _) in nums.iter().enumerate().take(nums.len() - 1) {
            max = max.max(i + nums[i] as usize);
            if i == end {
                end = max;
                step += 1;
            }
        }
        step
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_jump() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
