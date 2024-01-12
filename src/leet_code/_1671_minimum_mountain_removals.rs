use super::Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut suf = vec![0; n];
        let mut g = Vec::new();
        for (i, &x) in nums.iter().enumerate().rev() {
            let j = g.partition_point(|&v| v < x);
            if j == g.len() {
                g.push(x);
            } else {
                g[j] = x;
            }
            suf[i] = j + 1; // 从 nums[i] 开始的最长严格递减子序列的长度
        }

        let mut mx = 0;
        g.clear();
        for (i, &x) in nums.iter().enumerate() {
            let j = g.partition_point(|&v| v < x);
            if j == g.len() {
                g.push(x);
            } else {
                g[j] = x;
            }
            let pre = j + 1; // 在 nums[i] 结束的最长严格递增子序列的长度
            if pre >= 2 && suf[i] >= 2 {
                mx = mx.max(pre + suf[i] - 1); // 减去重复的 nums[i]
            }
        }
        (n - mx) as i32
    }

    pub fn minimum_mountain_removals_v1(nums: Vec<i32>) -> i32 {
        #[inline]
        fn subscript(dp: &mut Vec<i32>, x: i32) -> usize {
            match dp.binary_search(&x) {
                Ok(i) => i + 1,
                Err(i) => {
                    if i == dp.len() {
                        dp.push(x);
                    } else {
                        dp[i] = x;
                    }
                    i + 1
                }
            }
        }

        let mut dp = vec![];
        let left = nums.iter().map(|x| subscript(&mut dp, *x)).collect::<Vec<usize>>();

        dp.clear();
        let right = nums
            .iter()
            .rev()
            .map(|x| subscript(&mut dp, *x))
            .collect::<Vec<usize>>()
            .into_iter()
            .rev()
            .collect::<Vec<usize>>();

        let mut res = nums.len();
        for i in 0..nums.len() {
            if left[i] != 1 && right[i] != 1 {
                res = std::cmp::min(res, nums.len() + 1 - left[i] - right[i])
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_mountain_removals() {
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
        assert_eq!(Solution::minimum_mountain_removals(vec![4, 3, 2, 1, 1, 2, 3, 1]), 4);
        assert_eq!(Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]), 3);
    }

    #[test]
    fn test_minimum_mountain_removals_v1() {
        assert_eq!(Solution::minimum_mountain_removals_v1(vec![1, 3, 1]), 0);
        assert_eq!(Solution::minimum_mountain_removals_v1(vec![4, 3, 2, 1, 1, 2, 3, 1]), 4);
        assert_eq!(Solution::minimum_mountain_removals_v1(vec![2, 1, 1, 5, 6, 2, 3, 1]), 3);
    }
}
