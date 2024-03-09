use super::Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ret = i32::MIN;
        let mut ones_cnt = 0;
        let mut zeros_cnt = 0;
        for (i, &ch) in s.as_bytes().iter().enumerate() {
            if ch == b'1' {
                ones_cnt += 1
            } else {
                zeros_cnt += 1
            }
            if i != s.len() - 1 {
                ret = ret.max(zeros_cnt - ones_cnt)
            }
        }
        ones_cnt + ret
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::max_score("011101".to_owned()), 5);
        assert_eq!(Solution::max_score("00111".to_owned()), 5);
        assert_eq!(Solution::max_score("1111".to_owned()), 3);
    }
}
