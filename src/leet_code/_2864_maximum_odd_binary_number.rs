use super::Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let cnt = s.chars().filter(|&c| c == '1').count();
        "1".repeat(cnt - 1) + &"0".repeat(s.len() - cnt) + "1"
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::maximum_odd_binary_number("010".to_owned()), "001".to_owned());
        assert_eq!(Solution::maximum_odd_binary_number("0101".to_owned()), "1001".to_owned());
    }
}
