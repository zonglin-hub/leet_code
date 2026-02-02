use super::Solution;

impl Solution {
    pub fn max_operationss(s: String) -> i32 {
        let mut count_one = 0;
        let mut ans = 0;
        // i 位置点
        let mut i = 0;
        let chars = s.chars().collect::<Vec<char>>();
        let vec_len = chars.len();

        while i < vec_len {
            if chars[i] == '0' {
                while i + 1 < vec_len && chars[i + 1] == '0' {
                    i += 1;
                }
                ans += count_one;
            } else {
                count_one += 1;
            }
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_operations() {
        assert_eq!(Solution::max_operationss("1001101".to_string()), 4);
        assert_eq!(Solution::max_operationss("00111".to_owned()), 0);
    }
}
