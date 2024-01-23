use super::Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut first_index = 0;
        for i in 1..nums.len() {
            let len = (i - first_index + 1) as i32;
            if nums[i] - nums[first_index] == (len - 1) % 2 {
                res = res.max(len);
            } else if nums[i] - nums[i - 1] == 1 {
                first_index = i - 1;
                res = res.max(2);
            } else {
                first_index = i;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
    }
}
