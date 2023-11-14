use super::Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // 通过遍历数组，计算0和1的数量（c0和c1）
        let (c0, c1) = nums
            .iter()
            .fold((0_usize, 0_usize), |(c0, c1), &v| match v {
                0 => (c0 + 1, c1), // 如果遇到0，增加c0的计数
                1 => (c0, c1 + 1), // 如果遇到1，增加c1的计数
                _ => (c0, c1),     // 如果遇到2，计数保持不变
            });

        // 根据0和1的数量，将数组的前c0个元素填充为0，接下来的c1个元素填充为1，剩下的元素填充为2
        nums[0..c0].fill(0);
        nums[c0..c0 + c1].fill(1);
        nums[c0 + c1..].fill(2);
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
