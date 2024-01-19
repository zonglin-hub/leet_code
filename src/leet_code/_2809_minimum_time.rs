use super::Solution;

impl Solution {
    /// 函数式(动态规划)
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let n = nums1.len();
        let mut idx = (0..n).collect::<Vec<_>>();
        idx.sort_unstable_by_key(|&i| nums2[i]);
        let mut f = vec![0; n + 1];
        idx.into_iter().for_each(|i| {
            (1..=n).rev().for_each(|j| {
                f[j] = f[j].max(f[j - 1] + nums1[i] + nums2[i] * j as i32);
            });
        });
        f.iter()
            .enumerate()
            .position(|(i, val)| {
                nums1.iter().sum::<i32>() + nums2.iter().sum::<i32>() * i as i32 - val <= x
            })
            .map(|i| i as i32)
            .unwrap_or(-1)
    }

    /// 命令式
    pub fn minimum_time_v1(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let n = nums1.len();
        let mut idx = (0..n).collect::<Vec<_>>();
        idx.sort_by(|&i, &j| nums2[i].cmp(&nums2[j]));
        let mut f = vec![0; n + 1];
        for i in idx {
            for j in (1..=n).rev() {
                f[j] = f[j].max(f[j - 1] + nums1[i] + nums2[i] * j as i32);
            }
        }
        for (i, val) in f.iter().enumerate() {
            if nums1.iter().sum::<i32>() + nums2.iter().sum::<i32>() * i as i32 - val <= x {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_time() {
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
        assert_eq!(Solution::minimum_time(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
        assert_eq!(Solution::minimum_time(vec![4, 4, 9, 10], vec![4, 4, 1, 3], 16), 4);
        assert_eq!(
            Solution::minimum_time(vec![9, 2, 8, 3, 1, 9, 7, 6], vec![0, 3, 4, 1, 3, 4, 2, 1], 40),
            8
        );
    }

    #[test]
    fn test_minimum_time_v1() {
        assert_eq!(Solution::minimum_time_v1(vec![1, 2, 3], vec![1, 2, 3], 4), 3);
        assert_eq!(Solution::minimum_time_v1(vec![1, 2, 3], vec![3, 3, 3], 4), -1);
        assert_eq!(Solution::minimum_time_v1(vec![4, 4, 9, 10], vec![4, 4, 1, 3], 16), 4);
        assert_eq!(
            Solution::minimum_time_v1(
                vec![9, 2, 8, 3, 1, 9, 7, 6],
                vec![0, 3, 4, 1, 3, 4, 2, 1],
                40
            ),
            8
        );
    }
}
