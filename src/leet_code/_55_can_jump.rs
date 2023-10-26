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
