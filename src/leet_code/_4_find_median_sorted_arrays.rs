//! 寻找两个正序数组的中位数

use super::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
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
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.00000
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.50000
        );
    }

    #[test]
    fn test_append() {
        let mut vec = vec![1, 7, 3];
        let mut vec2 = vec![4, 5, 6];
        vec.append(&mut vec2);
        vec.sort();
        assert_eq!(vec, [1, 3, 4, 5, 6, 7]);
        assert_eq!(vec2, []);
    }
}
