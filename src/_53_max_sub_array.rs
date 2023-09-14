//! 最大子数组和

use crate::types::base_type::Solution;

impl Solution {
    pub fn max_sub_array_v1(nums: Vec<i32>) -> i32 {
        let (mut sum, mut res) = (0, nums[0]);
        for i in nums {
            sum = i.max(i + sum);
            res = res.max(sum)
        }
        res
    }

    // 使用迭代器的 fold 方法来实现
    pub fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter();

        // 在迭代器中，第一个元素需要被额外处理，因此我们使用了 unwrap_or 方法来提取第一个元素，如果 nums 是空的则使用默认值 0。
        let first = iter.next().unwrap_or(0);

        // 使用 into_iter 方法将 nums 转换为迭代器，并使用 fold 方法来遍历整个迭代器。
        // 在每一轮迭代过程中，我们将当前的元素 i 和之前的和 sum 进行比较，然后计算出新的和 sum 和当前的最大子序和 res，并将它们作为 (sum, res) 的新值进行返回。
        let (_, res) = iter.fold((first, first), |(mut sum, mut res), i| {
            sum = i.max(i + sum);
            res = res.max(sum);
            (sum, res)
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array_v1() {
        /*
            输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
            输出：6
            解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
        */
        assert_eq!(
            Solution::max_sub_array_v1(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );

        /*
            输入：nums = [1]
            输出：1
        */
        assert_eq!(Solution::max_sub_array_v1(vec![1]), 1);

        /*
            输入：nums = [5,4,-1,7,8]
            输出：23
        */
        assert_eq!(Solution::max_sub_array_v1(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn test_max_sub_array_v2() {
        assert_eq!(
            Solution::max_sub_array_v2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array_v2(vec![1]), 1);
        assert_eq!(Solution::max_sub_array_v2(vec![5, 4, -1, 7, 8]), 23);
    }
}
