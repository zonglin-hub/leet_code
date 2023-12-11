//! 赎金信

use super::Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut arr = [0; 26];

        ransom_note
            .as_bytes()
            .iter()
            .for_each(|i| arr[(*i - b'a') as usize] -= 1);

        magazine
            .as_bytes()
            .iter()
            .for_each(|i| arr[(*i - b'a') as usize] += 1);

        (0..26).all(|i| arr[i] >= 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_profit() {
        assert_eq!(
            Solution::is_subsequence("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("aa".to_string(), "ab".to_string()),
            false
        );
        assert!(Solution::is_subsequence(
            "aa".to_string(),
            "aab".to_string()
        ));
    }

    // #[test]
    // fn test_all() {
    //     let a = [1, 2, 3];

    //     assert!(a.iter().all(|&x| x > 0));

    //     assert!(!a.iter().all(|&x| x > 2));
    // }
}
