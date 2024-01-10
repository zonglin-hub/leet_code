use super::Solution;

use std::convert::TryInto;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut st = vec![];

        for c in s.chars() {
            st.push(c);

            let m = st.len();

            if m >= 2
                && (st[m - 2] == 'A' && st[m - 1] == 'B' || st[m - 2] == 'C' && st[m - 1] == 'D')
            {
                st.pop();
                st.pop();
            }
        }

        st.len().try_into().expect("类型转换异常")
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_length() {
        assert_eq!(Solution::min_length("ABFCACDB".to_owned()), 2);
        assert_eq!(Solution::min_length("ACBBD".to_owned()), 5);
    }
}
