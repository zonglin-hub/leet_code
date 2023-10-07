//! 最接近的三数之和

use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn three_sum_closest_v1(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = i32::MAX;
        let mut diff = i32::MAX;

        for i in 0..n {
            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                let new_diff = (sum - target).abs();

                if new_diff < diff {
                    diff = new_diff;
                    ans = sum;
                }

                match sum.cmp(&target) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => return sum,
                }
            }
        }
        ans
    }
}
