#![allow(unused)]
pub struct Solution;

impl Solution {
    /// 删除有序数组中的重复项
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        for j in (1..nums.len()) {
            if nums[j] != nums[j - 1] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }

    pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }

    pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
        let (mut p1, mut p2) = (0, 1);
        while p2 < nums.len() {
            match nums[p2] == nums[p1] {
                true => p2 += 1,
                false => {
                    p1 += 1;
                    // nums[p1] = nums[p2];
                    nums.swap(p1, p2);
                    p2 += 1;
                }
            }
        }
        p1 += 1;
        p1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2, 2, 2, 3, 4, 5, 5];
        let len = Solution::remove_duplicates_2(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(nums[..len as usize], [1, 2, 3, 4, 5]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = Solution::remove_duplicates(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(nums[..len as usize], [0, 1, 2, 3, 4]);
    }
}
