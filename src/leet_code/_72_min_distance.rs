use super::Solution;
use std::cmp;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let me = word1.to_lowercase();
        let t = word2.to_lowercase();

        let t_len = t.chars().count();
        if me.is_empty() {
            return t_len.try_into().unwrap();
        }
        if t.is_empty() {
            return me.chars().count().try_into().unwrap();
        }

        let mut dcol = (0..=t_len).collect::<Vec<_>>();
        let mut t_last = 0;

        for (i, sc) in me.chars().enumerate() {
            let mut current = i;
            dcol[0] = current + 1;

            for (j, tc) in t.chars().enumerate() {
                let next = dcol[j + 1];

                if sc == tc {
                    dcol[j + 1] = current;
                } else {
                    dcol[j + 1] = cmp::min(current, next);
                    dcol[j + 1] = cmp::min(dcol[j + 1], dcol[j]) + 1;
                }

                current = next;
                t_last = j;
            }
        }

        dcol[t_last + 1].try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_distance() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );

        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
