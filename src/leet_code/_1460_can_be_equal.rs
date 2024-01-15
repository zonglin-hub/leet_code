use super::Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_can_be_equal() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
        assert!(Solution::can_be_equal(vec![7], vec![7]));
        assert!(!Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
