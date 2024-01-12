use super::Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn check(s1: &[u8], s2: &[u8]) -> bool {
            let mut left_count = 0;
            s1.iter().chain(s2.iter()).all(|&b| match b {
                b'(' => {
                    left_count += 1;
                    true
                }
                b')' => {
                    if left_count == 0 {
                        false
                    } else {
                        left_count -= 1;
                        true
                    }
                }
                _ => true,
            })
        }

        fn get_info(s: &[u8]) -> (i32, i32) {
            let (mut invalid_left_count, mut invalid_right_count) = (0, 0);
            for &b in s.iter() {
                if b == b'(' {
                    invalid_left_count += 1;
                } else if b == b')' {
                    if invalid_left_count == 0 {
                        invalid_right_count += 1;
                    } else {
                        invalid_left_count -= 1;
                    }
                }
            }
            (invalid_left_count, invalid_right_count)
        }

        fn backtracing(
            s: &[u8],
            remove_left_count: i32,
            remove_right_count: i32,
            cur_result: &mut Vec<u8>,
            result: &mut Vec<String>,
        ) {
            if remove_left_count == 0 && remove_right_count == 0 {
                if check(&cur_result[..], s) {
                    let mut v = cur_result.to_vec();
                    v.extend_from_slice(s);
                    result.push(String::from_utf8(v).unwrap());
                }
                return;
            }

            let cur_len = cur_result.len();
            let n = s.len();
            let mut i = 0;
            while i < n {
                if remove_left_count + remove_right_count > (n - i) as i32 {
                    break;
                }

                if remove_left_count > 0 && s[i] == b'(' {
                    backtracing(
                        &s[i + 1..],
                        remove_left_count - 1,
                        remove_right_count,
                        cur_result,
                        result,
                    );
                } else if remove_right_count > 0 && s[i] == b')' {
                    backtracing(
                        &s[i + 1..],
                        remove_left_count,
                        remove_right_count - 1,
                        cur_result,
                        result,
                    );
                }
                cur_result.push(s[i]);

                while i + 1 < n && s[i + 1] == s[i] {
                    i += 1;
                    cur_result.push(s[i]);
                }
                i += 1;
            }
            cur_result.resize(cur_len, 0);
        }

        let (invalid_left_count, invalid_right_count) = get_info(s.as_bytes());
        let mut result = vec![];

        backtracing(
            s.as_bytes(),
            invalid_left_count,
            invalid_right_count,
            &mut vec![],
            &mut result,
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_remove_invalid_parentheses() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_string()),
            vec!["(())()".to_string(), "()()()".to_string()]
        );

        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_string()),
            vec!["(a())()".to_string(), "(a)()()".to_string()]
        );

        assert_eq!(Solution::remove_invalid_parentheses(")(".to_string()), vec!["".to_string()]);
    }
}
