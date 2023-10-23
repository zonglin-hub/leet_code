use super::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut step = 0;
        let mut max = 0;
        let mut end = 0;
        for i in 0..nums.len() - 1 {
            max = max.max(i + nums[i] as usize);
            if i == end {
                end = max;
                step += 1;
            }
        }
        step
    }
}
