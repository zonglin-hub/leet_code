#![allow(unused)]

use std::cmp::Ordering;
struct Solution;

impl Solution {
    /// 最接近的三数之和
    ///
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
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 10000), 0);
        assert_eq!(
            Solution::three_sum_closest(
                vec![
                    -13, 592, -501, 770, -952, -675, 322, -829, -246, 657, 608, 485, -112, 967,
                    -30, 182, -969, 559, -286, -64, 24, 365, -158, 701, 535, -429, -217, 28, 948,
                    -114, -536, -711, 693, 23, -958, -283, -700, -672, 311, 314, -712, -594, -351,
                    658, 747, 949, 70, 888, 166, 495, 244, -380, -654, 454, -281, -811, -168, -839,
                    -106, 877, -216, 523, -234, -8, 289, -175, 920, -237, -791, -976, -509, -4, -3,
                    298, -190, 194, -328, 265, 150, 210, 285, -176, -646, -465, -97, -107, 668,
                    892, 612, -54, -272, -910, 557, -212, -930, -198, 38, -365, -729, -410, 932, 4,
                    -565, -329, -456, 224, 443, -529, -428, -294, 191, 229, 112, -867, -163, -979,
                    236, -227, -388, -209, 984, 188, -549, 970, 951, -119, -146, 801, -554, 564,
                    -769, 334, -819, -356, -724, -219, 527, -405, -27, -759, 722, -774, 758, 394,
                    146, 517, 870, -208, 742, -782, 336, -364, -558, -417, 663, -914, 536, 293,
                    -818, 847, -322, 408, 876, -823, 827, 167
                ],
                7175
            ),
            2921
        );
    }
}
