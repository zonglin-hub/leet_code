use super::Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let valid = |s: &[u8]| s.iter().rev().take(s.len()).eq(s.iter().take(s.len()));
        let s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                /*
                    `i + 1..=j` 是一个 Rust 中的元组表示法，用来表示一个范围。在这个例子中，它表示从 `i + 1` 到 `j` 的所有整数，包括 `i + 1` 和 `j`。
                    例如，如果 `i = 0` 且 `j = 5`，那么 `0 + 1..=5` 就表示了从 `1` 到 `5` 的所有整数，即 `1, 2, 3, 4, 5`。
                */
                return valid(&s[i + 1..=j]) || valid(&s[i..=j - 1]);
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
