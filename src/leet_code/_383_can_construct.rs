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
