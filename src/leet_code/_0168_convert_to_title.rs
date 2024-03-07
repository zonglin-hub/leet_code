use super::Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut str = "".to_owned();
        let mut n = column_number;
        while n > 0 {
            let pop = ((n - 1) % 26) as u8;
            n = (n - 1) / 26;
            str.push((b'A' + pop) as char);
        }
        str.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::convert_to_title(1), "A".to_owned());
        assert_eq!(Solution::convert_to_title(28), "AB".to_owned());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_owned());
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW".to_owned());
    }
}
