use super::Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates.bytes().sum::<u8>() & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(!Solution::square_is_white("a1".to_owned()));
        assert!(Solution::square_is_white("h3".to_owned()));
        assert!(!Solution::square_is_white("c7".to_owned()));
    }
}
