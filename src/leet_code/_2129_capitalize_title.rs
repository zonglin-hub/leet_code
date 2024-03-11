use super::Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let words = title.split(' ').collect::<Vec<&str>>();
        let mut res = vec![];

        for word in words {
            if word.len() > 2 {
                // word[..1] 0 小于 1 但不包含 1
                // word[1..] 包含 1 与 无穷大
                res.push(word[..1].to_uppercase() + &word[1..].to_lowercase());
            } else {
                res.push(word.to_lowercase());
            }
        }
        res.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::capitalize_title("capiTalIze tHe titLe".to_owned()),
            "Capitalize The Title".to_owned()
        );
        assert_eq!(
            Solution::capitalize_title("First leTTeR of EACH Word".to_owned()),
            "First Letter of Each Word".to_owned()
        );
        assert_eq!(
            Solution::capitalize_title("i lOve leetcode".to_owned()),
            "i Love Leetcode".to_owned()
        );
    }
}
