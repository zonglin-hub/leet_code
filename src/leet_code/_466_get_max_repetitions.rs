use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let mut cnt = Vec::new();
        let mut hash = HashMap::new();
        let (mut i, mut k) = (0, 0);

        while i < n1 {
            for c in s1.chars() {
                if Some(c) == s2.chars().nth(k as usize % s2.len()) {
                    k += 1;
                }
            }

            cnt.push(k);

            if hash.contains_key(&(k % s2.len() as i32)) {
                let a = i - hash[&(k % s2.len() as i32)];
                let b = k - cnt[hash[&(k % s2.len() as i32)] as usize];
                let mut res = (n1 - i - 1) / a * b;

                for _ in 0..(n1 - i - 1) % a {
                    for c in s1.chars() {
                        if Some(c) == s2.chars().nth(k as usize % s2.len()) {
                            k += 1;
                        }
                    }
                }
                res += k;

                return res / s2.len() as i32 / n2;
            }

            hash.insert(k % s2.len() as i32, i);
            i += 1;
        }

        if cnt.is_empty() {
            return 0;
        }

        *cnt.last().unwrap() / s2.len() as i32 / n2
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_get_max_repetitions() {
        assert_eq!(
            Solution::get_max_repetitions("acb".to_owned(), 4, "ab".to_owned(), 2),
            2
        );
        assert_eq!(
            Solution::get_max_repetitions("acb".to_owned(), 1, "acb".to_owned(), 1),
            1
        );
    }
}
