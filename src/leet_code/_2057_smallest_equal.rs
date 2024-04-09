use super::Solution;

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for (idx, val) in nums.iter().enumerate().take(n) {
            if idx as i32 % 10 == *val {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::smallest_equal(vec![0, 1, 2]), 0);
        assert_eq!(Solution::smallest_equal(vec![4, 3, 2, 1]), 2);
        assert_eq!(Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), -1);
        assert_eq!(Solution::smallest_equal(vec![2, 1, 3, 5, 2]), 1);
    }
}
