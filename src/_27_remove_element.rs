#![allow(unused)]
pub struct Solution;

impl Solution {
    /// 移除元素
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[ans] = val;
                ans += 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    }
}
