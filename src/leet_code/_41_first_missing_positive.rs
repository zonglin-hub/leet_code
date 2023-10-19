use super::Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        for i in 0..l as usize {
            let mut n = nums[i];
            while n > 0 && n <= l && nums[(n - 1) as usize] != n {
                std::mem::swap(&mut nums[(n - 1) as usize], &mut n);
            }
        }

        for i in 0..l {
            if nums[i as usize] != i + 1 {
                return i + 1;
            }
        }
        l + 1
    }
}
