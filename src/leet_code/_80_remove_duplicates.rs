use super::Solution;

impl Solution {
    pub fn remove_duplicates_80(nums: &mut [i32]) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let (mut l, mut r) = (2, 2);
        while r < nums.len() {
            if nums[l - 2] != nums[r] {
                nums[l] = nums[r];
                l += 1;
            }
            r += 1;
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_remove_duplicates_80() {
        assert_eq!(Solution::remove_duplicates_80(&mut [1, 1, 1, 2, 2, 3]), 5);
        assert_eq!(Solution::remove_duplicates_80(&mut [0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
    }
}
