//! 三数之和

use std::cmp::Ordering;

use crate::Solution;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_v1() {
        /*
            输入：nums = [-1,0,1,2,-1,-4]
            输出：[[-1,-1,2],[-1,0,1]]
            解释：
            nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
            nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
            nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
            不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
            注意，输出的顺序和三元组的顺序并不重要。
        */
        assert_eq!(
            Solution::three_sum_v1(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );

        /*
            输入：nums = [0,1,1]
            输出：[]
            解释：唯一可能的三元组和不为 0 。
        */
        assert_eq!(
            Solution::three_sum_v1(vec![0, 1, 1]),
            Vec::<Vec<i32>>::new()
        );

        /*
            输入：nums = [0,0,0]
            输出：[[0,0,0]]
            解释：唯一可能的三元组和为 0 。
        */
        assert_eq!(Solution::three_sum_v1(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
