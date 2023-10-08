//! 寻找两个正序数组的中位数

use super::Solution;

impl Solution {
    pub fn find_median_sorted_arrays_v1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
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
