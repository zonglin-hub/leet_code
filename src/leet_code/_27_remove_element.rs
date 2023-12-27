//! 移除元素

use super::Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[right] != val {
                nums[left] = nums[right];
                left += 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::remove_element(&mut [3, 2, 2, 3], 3), 2);
        assert_eq!(
            Solution::remove_element(&mut [0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }
}
