//! 打家劫舍 IV

use super::Solution;

impl Solution {
    pub fn min_capability_2560(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_num = *nums.iter().max().expect("存在最大值");
        let mut min_num = *nums.iter().min().expect("存在最小值");
        while min_num <= max_num {
            let middle = (min_num + max_num) / 2;
            let mut count = 0;
            let mut visited = false;
            for &x in nums.iter() {
                if x <= middle && !visited {
                    count += 1;
                    visited = true;
                } else {
                    visited = false;
                }
            }

            if count >= k {
                max_num = middle - 1;
            } else {
                min_num = middle + 1;
            }
        }

        min_num
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_capability() {
        assert_eq!(Solution::min_capability_2560(vec![2, 3, 5, 9], 2), 5);
        assert_eq!(Solution::min_capability_2560(vec![2, 7, 9, 3, 1], 2), 2);
    }
}
