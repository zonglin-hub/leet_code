use super::Solution;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.iter().map(|x| *x as i64).collect::<Vec<_>>();
        let mut suf = vec![0_i64; nums.len() + 1];
        for (i, &v) in nums.iter().enumerate().rev() {
            suf[i] = suf[i + 1] | v;
        }
        let mut ans = 0_i64;
        let mut pre = 0_i64;
        for (i, &v) in nums.iter().enumerate() {
            ans = ans.max(pre | v << k | suf[i + 1]);
            pre |= v;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_maximum_or() {
        assert_eq!(Solution::maximum_or(vec![12, 9], 1), 30);
        assert_eq!(Solution::maximum_or(vec![8, 1, 2], 2), 35);
    }
}
