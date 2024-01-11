use super::Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let n = word.len();
        let mut cnt = 1;
        let b = word.as_bytes();
        for i in 1..n {
            if b[i - 1] >= b[i] {
                cnt += 1;
            }
        }
        (cnt * 3 - n) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_add_minimum() {
        assert_eq!(Solution::add_minimum("b".to_owned()), 2);
        assert_eq!(Solution::add_minimum("aaa".to_owned()), 6);
        assert_eq!(Solution::add_minimum("abc".to_owned()), 0);
    }
}
