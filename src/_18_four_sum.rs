#![allow(unused)]
pub struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/4sum/
    ///
    /// 四数之和
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::k_sum(4, nums, target)
    }

    // 使用双指针的方式优化 k_sum 函数。
    // 当 k > 2 时，可以使用类似于两数之和的双指针方式，将问题转化为更小的 k-1 的子问题，然后进行递归。
    // 具体来说，在双指针的循环内部，需要判断当前的值是否与前一个值相等，如果相等则将指针移动到下一个不相等的位置，以避免出现重复的结果
    // 代码先判断是否可以找到 k 个数的组合，然后对于 k=2 的情况，使用双指针方式寻找两数之和等于 target 的组合。对于 k>2 的情况，使用递归的方式，将问题转化为更小的 k-1 的子问题，再进行求解。最后将结果返回即可。
    pub fn k_sum(k: i32, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // // 首先判断数组长度是否小于 k，小于 k 则无法找到 k 个数的组合
        if nums.len() < k as usize {
            return Vec::new();
        }

        let mut results = Vec::new();

        // 将数组排序，方便双指针遍历
        let mut nums = nums;
        nums.sort();

        if k == 2 {
            // 当 k=2 时，使用双指针方式寻找两数之和等于 target 的组合
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[left] + nums[right];
                if sum == target {
                    // 找到一个符合要求的组合，将其加入结果中
                    results.push(vec![nums[left], nums[right]]);
                    left += 1;

                    // 移动左指针，直到找到一个与前一个值不相等的位置
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                } else if sum < target {
                    // 如果和小于 target，则移动左指针
                    left += 1;
                } else {
                    // 如果和大于 target，则移动右指针
                    right -= 1;
                }
            }
        } else {
            for i in 0..nums.len() - k as usize + 1 {
                if nums[i] * k as i32 > target {
                    // 如果当前值乘以 k 大于 target，则后面的数无论如何都无法满足条件，直接退出循环
                    break;
                }
                if i > 0 && nums[i] == nums[i - 1] {
                    // 如果当前值与前一个值相等，则跳过
                    continue;
                }

                // 找到 k-1 个数的组合，使其和为 target 减去当前值
                let sub_results = Self::k_sum(k - 1, nums[i + 1..].to_vec(), target - nums[i]);
                for mut r in sub_results {
                    // 将当前值加入每个组合的末尾，即可得到 k 个数的组合
                    r.push(nums[i]);
                    results.push(r);
                }
            }
        }
        results
    }

    // pub fn k_sum_1(k: i32, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    //     if nums.len() < k as usize {
    //         return Vec::new();
    //     }

    //     let mut nums = nums;
    //     nums.sort();

    //     match k {
    //         _ if k != 2 => {
    //             let len = nums.len() - 1;
    //             let mut res = Vec::new();

    //             for i in 0..=len - 2 {
    //                 if i != 0 && nums[i - 1] == nums[i] {
    //                     continue;
    //                 } else if target / k >= nums[i] && target / k <= nums[len] {
    //                     let t = target.saturating_add(-nums[i]);
    //                     let k_1_sum = Self::k_sum(k - 1, nums[i + 1..].to_vec(), t);

    //                     if !k_1_sum.is_empty() {
    //                         res.append(
    //                             &mut k_1_sum
    //                                 .into_iter()
    //                                 .map(|mut s| {
    //                                     s.push(nums[i]);
    //                                     s
    //                                 })
    //                                 .collect::<Vec<Vec<i32>>>(),
    //                         );
    //                     }
    //                 }
    //             }

    //             res
    //         }
    //         2 => Self::two_sum(nums, target),
    //         _ => Vec::new(),
    //     }
    // }

    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    //     let mut left = 0;
    //     let mut right = nums.len() - 1;
    //     let mut result = Vec::new();
    //     while left < right {
    //         let l_value = nums[left];
    //         let r_value = nums[right];
    //         if left != 0 && l_value == nums[left - 1] {
    //             left += 1;
    //             continue;
    //         }
    //         if right != nums.len() - 1 && r_value == nums[right + 1] {
    //             right -= 1;
    //             continue;
    //         }
    //         match (l_value + r_value - target).signum() {
    //             1 => right -= 1,
    //             0 => {
    //                 result.push([l_value, r_value].to_vec());
    //                 left += 1
    //             }
    //             _ => left += 1,
    //         }
    //     }
    //     result
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![1, 2, -1, -2], vec![0, 2, 0, -2], vec![0, 1, 0, -1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
    }
}
