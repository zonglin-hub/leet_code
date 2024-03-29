use super::Solution;

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mn = 1000;
        let mut left = vec![0; n];
        for i in 1..n {
            mn = nums[i - 1].min(mn);
            left[i] = mn;
        }

        let mut res = 1000;
        let mut right = nums[n - 1];
        for i in (1..n - 1).rev() {
            if left[i] < nums[i] && nums[i] > right {
                res = res.min(left[i] + nums[i] + right);
            }
            right = right.min(nums[i]);
        }
        
        if res < 1000 {
            res
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
