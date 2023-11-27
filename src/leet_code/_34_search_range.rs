//! 在排序数组中查找元素的第一个和最后一个位置

use super::Solution;

impl Solution {
    /// 二分
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.partition_point(|&x| x < target) as i32;
        let r = nums.partition_point(|&x| x <= target) as i32;
        match l < r {
            true => vec![l, r - 1],
            false => vec![-1, -1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_search_range() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    fn test_partition_point() {
        let v = [1, 2, 3, 3, 5, 6, 7];
        let i = v.partition_point(|&x| x < 5);

        assert_eq!(i, 4);
        assert!(v[..i].iter().all(|&x| x < 5));
        assert!(v[i..].iter().all(|&x| !(x < 5)));
    }
}
