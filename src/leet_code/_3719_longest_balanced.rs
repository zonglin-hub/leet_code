use std::collections::HashMap;

use crate::leet_code::Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        for i in 0..nums.len() {
            let mut odd = HashMap::new();
            let mut even = HashMap::new();

            for j in i..nums.len() {
                // 在二进制中：
                // 偶数的最后一位是 0：2 (10), 4 (100), 6 (110), ...
                // 奇数的最后一位是 1：1 (1), 3 (11), 5 (101), 7 (111), ...
                // & 1（按位与 1）只保留最后一位：
                // 偶数 & 1 = 0
                // 奇数 & 1 = 1
                let map = if nums[j] & 1 == 1 { &mut odd } else { &mut even };

                *map.entry(nums[j]).or_insert(0) += 1;

                if odd.len() == even.len() {
                    max_len = max_len.max((j - i + 1) as i32)
                }
            }
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_longest_balanced() {
        assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
        assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
        assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
