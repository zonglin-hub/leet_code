use super::Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;

        for i in 0..31 {
            let mut cnt = 0;
            for &num in nums.iter() {
                if (num >> i) & 1 == 1 {
                    cnt += 1;
                }
            }
            if cnt >= k {
                ans |= 1 << i;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
        assert_eq!(Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
        assert_eq!(Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
}
