//! 三数之和

use std::cmp::Ordering;

use super::Solution;

impl Solution {
    pub fn three_sum_v1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        nums.sort();
        let mut result = vec![];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[l], nums[r]]);

                        l += 1;

                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }

                        r -= 1;

                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                }
            }
        }

        result
    }
}
