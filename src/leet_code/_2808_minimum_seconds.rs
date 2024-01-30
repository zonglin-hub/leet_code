use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            m.entry(x).or_insert(vec![i as i32]).push(i as i32);
        }
        let n = nums.len();
        m.iter().fold(n as i32, |l, (_, v)| {
            let nn = v.len() - 1;
            let mut mx = v[0] + n as i32 - v[nn];
            for i in 1..=nn {
                mx = mx.max(v[i] - v[i - 1]);
            }
            l.min(mx / 2)
        })
    }

    pub fn minimum_seconds_command(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            m.entry(x).or_insert(vec![i as i32]).push(i as i32);
        }
        let n = nums.len() as i32;
        let mut min_max_diff = n; // 初始化为n，因为差值不可能大于n
        for (_, v) in m {
            let nn = v.len() - 1;
            let mut mx = v[0] + n - v[nn];
            for i in 1..=nn {
                mx = mx.max(v[i] - v[i - 1]);
            }
            min_max_diff = min_max_diff.min(mx / 2);
        }
        min_max_diff
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_seconds() {
        assert_eq!(Solution::minimum_seconds(vec![1, 2, 1, 2]), 1);
        assert_eq!(Solution::minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
        assert_eq!(Solution::minimum_seconds(vec![5, 5, 5, 5]), 0);
        assert_eq!(Solution::minimum_seconds(vec![8, 13, 3, 3]), 1);
    }

    #[test]
    fn test_minimum_seconds_command() {
        assert_eq!(Solution::minimum_seconds_command(vec![1, 2, 1, 2]), 1);
        assert_eq!(Solution::minimum_seconds_command(vec![2, 1, 3, 3, 2]), 2);
        assert_eq!(Solution::minimum_seconds_command(vec![5, 5, 5, 5]), 0);
        assert_eq!(Solution::minimum_seconds_command(vec![8, 13, 3, 3]), 1);
    }
}
