use super::Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut a: Vec<i32> = nums.clone();
        let mut ans: i32 = 0;
        if nums.len() < 2 {
            return 0;
        }
        let b: i32 = nums[0] + nums[1];
        while a.len() > 1 {
            if a[0] + a[1] == b {
                a.remove(0);
                a.remove(0);
                ans += 1;
            } else {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_operations() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 1);
    }
}
