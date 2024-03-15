use super::Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut valid = true;
        let mut res = 0;

        for c in s.chars() {
            if c == '|' {
                valid = !valid;
            } else if c == '*' && valid {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::count_asterisks("l|*e*et|c**o|*de|".to_owned()), 2);
        assert_eq!(Solution::count_asterisks("iamprogrammer".to_owned()), 0);
        assert_eq!(Solution::count_asterisks("yo|uar|e**|b|e***au|tifu|l".to_owned()), 5);
    }
}
