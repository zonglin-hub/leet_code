//! 四数之和

use super::Solution;

use std::cmp::Ordering;

fn it(nums: &[i32], i: usize, n: usize) -> i64 {
    (nums[i..i + n]).iter().map(|x| *x as i64).sum::<i64>()
}

fn its(nums: &[i32], length: usize, n: usize) -> i64 {
    (nums[length - n..length])
        .iter()
        .map(|x| *x as i64)
        .sum::<i64>()
}

fn bo(nums: &[i32], n: usize) -> bool {
    nums[n] == nums[n - 1]
}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut quadruplets = vec![];
        let target = target as i64;
        let length = nums.len();

        if length < 4 {
            return quadruplets;
        }

        nums.sort_unstable();

        for i in 0..length - 3 {
            // i > 0 && nums[i] == nums[i - 1]
            if i > 0 && bo(&nums, i) {
                continue; // 去重
            }

            // nums[i + 1] + nums[i + 2] + nums[i + 3]
            if it(&nums, i, 4) > target {
                break; // 剪枝
            }

            // nums[i] + nums[length - 3] + nums[length - 2] + nums[length - 1]
            if nums[i] as i64 + its(&nums, length, 3) < target {
                continue;
            }

            for j in (i + 1)..length - 2 {
                if j > i + 1 && bo(&nums, j) {
                    continue;
                }

                if nums[i] as i64 + it(&nums, j, 3) > target {
                    break;
                }

                if (nums[i] + nums[j]) as i64 + its(&nums, length, 2) < target {
                    continue;
                }

                let mut left = j + 1;
                let mut right = length - 1;

                while left < right {
                    match ((nums[i] + nums[j] + nums[left] + nums[right]) as i64).cmp(&target) {
                        Ordering::Less => left += 1,
                        Ordering::Greater => right -= 1,
                        Ordering::Equal => {
                            quadruplets.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            right -= 1;
                            left += 1;

                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }

                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1;
                            }
                        }
                    }
                }
            }
        }
        quadruplets
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_four_sum() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
        assert_eq!(
            Solution::four_sum(
                vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000],
                1000000000
            ),
            vec![vec![0, 0, 0, 1000000000]]
        );
    }
}
