use super::Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (c0, c1) = nums
            .iter()
            .fold((0_usize, 0_usize), |(c0, c1), &v| match v {
                0 => (c0 + 1, c1),
                1 => (c0, c1 + 1),
                _ => (c0, c1),
            });

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
