use super::Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        fn get(nums: &[i32], idx: usize) -> Vec<i32> {
            if idx as i32 == -1_i32 || idx == nums.len() {
                return vec![0, 0];
            }

            vec![1, nums[idx]]
        }

        fn compare(nums: &[i32], idx1: usize, idx2: usize) -> i32 {
            let (num1, num2) = (get(nums, idx1), get(nums, idx2));

            if num1[0] != num2[0] {
                return if num1[0] > num2[0] { 1 } else { -1 };
            }

            match num1[1].cmp(&num2[1]) {
                Ordering::Equal => 0,
                Ordering::Greater => 1,
                Ordering::Less => -1,
            }
        }

        let mut ans = -1_i32;
        let (mut left, mut right) = (0, nums.len() - 1);

        while left <= right {
            let idx = (left + right) / 2;
            let x = compare(&nums, idx, idx + 1);

            if compare(&nums, idx - 1, idx) < 0 && x > 0 {
                ans = idx as i32;
                break;
            }

            if x < 0 {
                left = idx + 1;
            } else {
                right = idx - 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 1]), 1);
    }
}
