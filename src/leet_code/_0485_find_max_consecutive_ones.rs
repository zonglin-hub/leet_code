use super::Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&x| x == 0).map(|x| x.len()).max().unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
