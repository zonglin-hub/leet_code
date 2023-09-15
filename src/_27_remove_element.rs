//! 移除元素

use crate::Solution;

impl Solution {
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
