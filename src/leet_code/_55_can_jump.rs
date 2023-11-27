use super::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        nums.into_iter()
            .enumerate()
            .try_fold(0, |d, (i, n)| {
                if d >= i {
                    Some(d.max(i + n as usize))
                } else {
                    None
                }
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_55_can_jump() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
