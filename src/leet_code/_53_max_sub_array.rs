//! 最大子数组和

use super::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut sum, mut res) = (0, nums[0]);
        for i in nums {
            sum = i.max(i + sum);
            res = res.max(sum)
        }
        res
    }

    // pub fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
    //     let mut iter = nums.into_iter();
    //     let first = iter.next().unwrap_or(0);
    //     let (_, res) = iter.fold((first, first), |(mut sum, mut res), i| {
    //         sum = i.max(i + sum);
    //         res = res.max(sum);
    //         (sum, res)
    //     });
    //     res
    // }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    // #[test]
    // fn test_max_sub_array_v2() {
    //     assert_eq!(
    //         Solution::max_sub_array_v2(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
    //         6
    //     );
    //     assert_eq!(Solution::max_sub_array_v2(vec![1]), 1);
    //     assert_eq!(Solution::max_sub_array_v2(vec![5, 4, -1, 7, 8]), 23);
    // }
}
