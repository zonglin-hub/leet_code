use super::Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        let n = s.len();
        for i in 0..n {
            let mut words = 'a'..='z';
            if chars[i] == '?' {
                let left = if i == 0 { None } else { Some(chars[i - 1]) };
                let right = if i == n - 1 { None } else { Some(chars[i + 1]) };
                if let Some(w) = words.find(|&w| Some(w) != left && Some(w) != right) {
                    chars[i] = w;
                }
            }
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::modify_string("?zs".to_owned()), "azs".to_owned());
        assert_eq!(Solution::modify_string("ubv?w".to_owned()), "ubvaw".to_owned());
    }
}
