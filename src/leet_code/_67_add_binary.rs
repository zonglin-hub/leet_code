use super::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!(
            "{:b}",
            (i128::from_str_radix(&a, 2).unwrap() + i128::from_str_radix(&b, 2).unwrap())
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );

        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    // #[test]
    // #[ignore = "insignificant"]
    // fn test_() {
    //     let b = "10"; // 二进制
    //     let a = i128::from_str_radix(&b, 2).unwrap();
    //     println!("{}", a); // 十进制
    //     println!("{:b}", a); // 二进制
    // }
}
