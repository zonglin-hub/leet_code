use super::Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.iter().zip(nums.iter().skip(1)).all(|(&i, &j)| j >= i)
            || nums.iter().zip(nums.iter().skip(1)).all(|(&i, &j)| j <= i)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_score_of_parentheses_v1() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }
}
