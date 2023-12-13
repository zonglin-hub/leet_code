//! 寻找两个正序数组的中位数

use std::cmp::Ordering;

use super::Solution;

impl Solution {
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
            return Self::find_kth_smallest(nums2, nums1, k);
        }

        if nums1.is_empty() {
            return nums2[k];
        }

        let mid = nums1.len() / 2;
        let left_k = Self::find_first_greater_or_equal(nums2, nums1[mid]);

        match (mid as i32 + left_k as i32).cmp(&(k as i32)) {
            Ordering::Less => Self::find_kth_smallest(&nums1[mid + 1..], nums2, k - mid - 1),
            Ordering::Greater => Self::find_kth_smallest(&nums1[..mid], nums2, k),
            Ordering::Equal => nums1[mid],
        }
    }

    /// 二分
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_length = nums1.len() + nums2.len();
        let middle = total_length / 2;
        let kth_smallest = Self::find_kth_smallest(&nums1, &nums2, middle);

        match total_length % 2 {
            0 => (kth_smallest + Self::find_kth_smallest(&nums1, &nums2, middle - 1)) as f64 / 2.0,
            _ => kth_smallest as f64,
        }
    }

    // /// 合并
    // pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    //     nums1.append(&mut nums2);
    //     nums1.sort();

    //     let n = nums1.len();
    //     match n & 1 != 0 {
    //         true => nums1[n / 2] as f64,
    //         false => (nums1[n / 2] + nums1[n / 2 - 1]) as f64 / 2.0,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.00000
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.50000
        );
    }

    // #[test]
    // fn test_append() {
    //     let mut vec = vec![1, 7, 3];
    //     let mut vec2 = vec![4, 5, 6];
    //     vec.append(&mut vec2);
    //     vec.sort();
    //     assert_eq!(vec, [1, 3, 4, 5, 6, 7]);
    //     assert_eq!(vec2, []);
    // }
}
