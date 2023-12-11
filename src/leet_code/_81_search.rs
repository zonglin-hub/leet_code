use super::Solution;

impl Solution {
    pub fn search_v2(nums: Vec<i32>, target: i32) -> bool {
        nums.contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_search() {
        assert!(Solution::search_v2(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert_eq!(Solution::search_v2(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
