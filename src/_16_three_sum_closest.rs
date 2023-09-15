//! 最接近的三数之和

use std::cmp::Ordering;

use crate::Solution;

impl Solution {
    /// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。返回这三个数的和。
    ///
    /// # Approach
    ///
    /// - 以非降序对输入数组进行排序。
    /// - 对于数组中的每个元素，执行两个指针搜索以找到总和最接近目标-当前元素的另外两个元素。
    /// - 如果当前总和正好等于目标，立即返回。
    ///
    /// Time complexity: O(n^2)\
    /// Space complexity: O(1)
    ///
    /// # 优化 替换 sort | sort_unstable
    /// sort函数是稳定排序算法，它保证相等的元素之间的顺序不会改变。而sort_unstable函数则是非稳定排序算法，它不保证相等的元素之间的顺序不会改变。
    /// 在这段代码中，我们并不关心相等元素之间的顺序，只关心排序后的元素是否满足我们的目标，排序的顺序不影响结果的正确性。
    /// 在相同时间复杂度下，非稳定排序算法比稳定排序算法获得更好的性能，因此使用sort_unstable函数可以进一步提高代码的性能。
    pub fn three_sum_closest_v1(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let (mut ans, mut diff) = (i32::MAX, i32::MAX);
        for i in 0..n {
            let (mut l, mut r) = (i + 1, n - 1);
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
