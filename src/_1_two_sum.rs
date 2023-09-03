#![allow(unused)]
pub struct Solution;

impl Solution {
    /// 两数之和
    ///
    /// 这是一道经典的数组遍历和哈希表查找的问题。
    /// 题目要求在给定的数组中找出两个数的和为目标数，返回这两个数的下标。
    /// 因此，可以通过遍历数组，依次判断每个数和目标数的差是否已经存在于哈希表中，如果存在，则找到了符合条件的两个数，返回它们的下标即可。
    /// 如果差不存在于哈希表中，则将当前数及其下标插入哈希表中，等待后续的查找。
    /// 如果遍历完整个数组都没有找到符合条件的两个数，则返回空数组。
    /// 这样，时间复杂度为O(n)，空间复杂度为O(n)，其中n为数组的长度。
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 如果数组长度为1，并且数组中的元素和目标值相等，则返回空的数组
        if nums.len() == 1 && nums[0] == target {
            return vec![0];
        }
        // 创建一个HashMap，用于存储数组中的元素和元素的索引
        let mut map = std::collections::HashMap::new();
        // 遍历数组中的每一个元素
        for (k, v) in nums.into_iter().enumerate() {
            // 如果map中存在target - v的值，则返回元素和索引
            if let Some(&a) = map.get(&(target - v)) {
                return vec![a, k as i32];
            }
            // 否则，将元素和元素的索引添加到map中
            map.insert(v, k as i32);
        }
        // 如果map中没有找到target - v的值，则返回空的数组
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![], 9), vec![]);
        assert_eq!(Solution::two_sum(vec![9], 9), vec![0]);
        assert_eq!(Solution::two_sum(vec![8], 9), vec![]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
