use super::Solution;

impl Solution {
    /// 这个函数是一个用于合并两个有序数组的程序。函数的输入是两个已经排序的数组`nums1`和`nums2`，以及它们的长度`m`和`n`。函数的输出是一个新的排序后的数组。
    ///
    /// 函数首先将`nums2`中的元素复制到`nums1`的末尾，然后使用`sort()`方法对`nums1`进行排序。
    #[allow(clippy::ptr_arg)]
    pub fn merge_88(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        (0..n).for_each(|i| nums1[(m + i) as usize] = nums2[i as usize]);
        nums1.sort()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_merge() {
        let mut num1 = vec![1, 2, 3, 0, 0, 0];
        Solution::merge_88(&mut num1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(num1, vec![1, 2, 2, 3, 5, 6]);

        let mut num1 = vec![1];
        Solution::merge_88(&mut num1, 1, &mut vec![], 0);
        assert_eq!(num1, vec![1]);

        let mut num1 = vec![0];
        Solution::merge_88(&mut num1, 0, &mut vec![1], 1);
        assert_eq!(num1, vec![1]);
    }
}
