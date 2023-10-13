//! 寻找两个正序数组的中位数

use super::Solution;

impl Solution {
    pub fn find_median_sorted_arrays_v2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();

        let n = nums1.len();
        match n & 1 != 0 {
            // 奇数
            true => nums1[n / 2] as f64,
            // 偶数
            false => (nums1[n / 2] + nums1[n / 2 - 1]) as f64 / 2.0,
        }
    }
}
