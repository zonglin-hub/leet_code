//! 寻找两个正序数组的中位数

use std::cmp::Ordering;

use super::Solution;

impl Solution {
    /// 划分数组 (力扣官方题解)
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays_v2(nums2, nums1);
        }

        let m = nums1.len();
        let n = nums2.len();
        let mut left = 0;
        let mut right = m;
        let mut median1 = 0;
        let mut median2 = 0;

        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;

            let nums_im1 = match i == 0 {
                true => i32::MIN,
                false => nums1[i - 1],
            };

            let nums_i = match i == m {
                true => i32::MAX,
                false => nums1[i],
            };

            let nums_jm1 = match j == 0 {
                true => i32::MIN,
                false => nums2[j - 1],
            };

            let nums_j = match j == n {
                true => i32::MAX,
                false => nums2[j],
            };

            if nums_im1 <= nums_j {
                median1 = nums_im1.max(nums_jm1);
                median2 = nums_i.min(nums_j);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if (m + n) % 2 == 0 {
            (median1 + median2) as f64 / 2.0
        } else {
            median1 as f64
        }
    }

    /// 二分查找 (力扣官方题解)
    pub fn find_median_sorted_arrays_v1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        /// 寻找数组中第一个大于等于给定值的索引
        fn find_first_greater_or_equal(nums: &[i32], target: i32) -> usize {
            let (mut i, mut j) = (0, nums.len());

            while i < j {
                let mid = (i + j) / 2;
                match nums[mid] < target {
                    true => i = mid + 1,
                    false => j = mid,
                }
            }
            i
        }

        /// 寻找两个数组中的第 k 个最小值
        fn find_kth_smallest(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
            if nums1.len() > nums2.len() {
                return find_kth_smallest(nums2, nums1, k);
            }

            if nums1.is_empty() {
                return nums2[k];
            }

            let mid = nums1.len() / 2;
            let left_k = find_first_greater_or_equal(nums2, nums1[mid]);

            match (mid as i32 + left_k as i32).cmp(&(k as i32)) {
                Ordering::Less => find_kth_smallest(&nums1[mid + 1..], nums2, k - mid - 1),
                Ordering::Greater => find_kth_smallest(&nums1[..mid], nums2, k),
                Ordering::Equal => nums1[mid],
            }
        }

        let total_length = nums1.len() + nums2.len();
        let middle = total_length / 2;
        let kth_smallest = find_kth_smallest(&nums1, &nums2, middle);

        match total_length % 2 {
            0 => (kth_smallest + find_kth_smallest(&nums1, &nums2, middle - 1)) as f64 / 2.0,
            _ => kth_smallest as f64,
        }
    }

    /// 合并
    pub fn find_median_sorted_arrays_v2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();

        let n = nums1.len();
        match n & 1 != 0 {
            true => nums1[n / 2] as f64,
            false => (nums1[n / 2] + nums1[n / 2 - 1]) as f64 / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.00000);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.50000);
    }

    #[test]
    fn test_find_median_sorted_arrays_v1() {
        assert_eq!(Solution::find_median_sorted_arrays_v1(vec![1, 3], vec![2]), 2.00000);
        assert_eq!(Solution::find_median_sorted_arrays_v1(vec![1, 2], vec![3, 4]), 2.50000);
    }

    #[test]
    fn test_find_median_sorted_arrays_v2() {
        assert_eq!(Solution::find_median_sorted_arrays_v2(vec![1, 3], vec![2]), 2.00000);
        assert_eq!(Solution::find_median_sorted_arrays_v2(vec![1, 2], vec![3, 4]), 2.50000);
    }
}
