#![allow(unused)]
struct Solution;

impl Solution {
    /// 寻找两个正序数组的中位数
    ///
    /// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
    /// 算法的时间复杂度应该为 O(log (m+n)) 。
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len() + nums2.len();
        let (mut i, mut j) = (0, 0);
        let (mut r1, mut r2) = (0, 0);

        while (i + j) < (n / 2 + 1) {
            let mid = match (nums1.get(i), nums2.get(j)) {
                (Some(&x), Some(&y)) => {
                    if x > y {
                        j += 1;
                        y
                    } else {
                        i += 1;
                        x
                    }
                }

                (Some(&x), None) => {
                    i += 1;
                    x
                }

                (None, Some(&y)) => {
                    j += 1;
                    y
                }

                (None, None) => {
                    unreachable!()
                }
            };

            r1 = r2;
            r2 = mid;
        }

        if n % 2 == 0 {
            return (r1 as f64 + r2 as f64) / 2.0;
        }

        r2 as f64
    }
}

mod tests {
    use super::*;

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
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
            0.00000
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1]),
            1.00000
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2], vec![]),
            2.00000
        );

        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for i in 1..10001 {
            if i % 2 == 0 {
                v1.push(i);
            } else {
                v2.push(i);
            }
        }
        let ans = Solution::find_median_sorted_arrays(v1, v2);
        assert!((ans - 5000.5).abs() < 0.0001);
    }
}
