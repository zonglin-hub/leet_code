pub struct Solution;

impl Solution {
    ///
    /// 正常情况： vec![1, 2, 3] 的下一个排列为 vec![1, 3, 2]
    /// 特殊情况： vec![3, 2, 1] 的下一个排列为 vec![1, 2, 3]
    /// 边缘情况： vec![1, 1, 5] 的下一个排列为 vec![1, 5, 1]
    /// 单元素情况： vec![1] 的下一个排列为 vec![1] （不存在下一个排列）
    /// 
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 寻找第一个比当前元素大的元素
        let mut n = nums.len() - 1;
        while n > 0 && nums[n - 1] >= nums[n] {
            n -= 1;
        }

        // 如果没有找到，则直接返回
        if n == 0 {
            nums.reverse();
            return;
        }

        // 寻找第一个比当前元素大的元素的前一个元素
        let mut j = nums.len() - 1;
        while j >= n && nums[n - 1] >= nums[j] {
            j -= 1;
        }

        // 交换当前元素和前一个元素
        nums.swap(n - 1, j);
        // 对剩余元素进行排序
        nums[n..].sort_unstable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 3, 2]);
    }
}
