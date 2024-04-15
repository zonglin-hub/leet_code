use super::Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut count = 0;
        let mut ans = String::from("");
        let mut n = n;

        while n > 0 {
            let cur = n % 10;
            n /= 10;
            ans.push_str(&cur.to_string());
            count += 1;
            if count % 3 == 0 && n > 0 {
                ans.push('.')
            }
        }

        ans.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::thousand_separator(987), "987".to_owned());
        assert_eq!(Solution::thousand_separator(1234), "1.234".to_owned());
        assert_eq!(Solution::thousand_separator(123456789), "123.456.789".to_owned());
        assert_eq!(Solution::thousand_separator(0), "".to_owned());
    }
}
