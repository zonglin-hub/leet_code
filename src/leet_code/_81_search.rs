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
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 0;
        let out = Solution::search_v2(nums, target);
        assert!(out);

        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        let target = 3;
        let out = Solution::search_v2(nums, target);
        assert_eq!(out, false);
    }
}
