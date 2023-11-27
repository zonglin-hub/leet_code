//! 括号的分数

use super::Solution;

impl Solution {
    pub fn score_of_parentheses_v1(s: String) -> i32 {
        let mut score = Vec::with_capacity((s.len() >> 1) + 1);
        score.push(0);
        for c in s.bytes() {
            if c == b'(' {
                score.push(0);
            } else {
                let last = score.pop().expect("");
                *score.last_mut().expect("") += if last == 0 { 1 } else { last << 1 };
            }
        }

        score[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_score_of_parentheses_v1() {
        assert_eq!(Solution::score_of_parentheses_v1("()".to_string()), 1);
        assert_eq!(Solution::score_of_parentheses_v1("(())".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses_v1("()()".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses_v1("(()(()))".to_string()), 6);
    }
}
