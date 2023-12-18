use super::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        fn vals(i: i32, a: &str) -> u8 {
            if i >= 0 {
                a.as_bytes()[i as usize] - b'0'
            } else {
                0
            }
        }

        let mut res = vec![];
        let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);
        let mut carry = 0;

        while i >= 0 || j >= 0 || carry > 0 {
            let a = vals(i, &a);
            let b = vals(j, &b);
            let sum = a ^ b ^ carry;

            carry = (a & b) | (a & carry) | (b & carry);
            res.push((sum + b'0') as char);

            i -= 1;
            j -= 1;
        }
        res.iter().rev().collect()
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
            Solution::add_binary("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string(), "1".to_string()),
            "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
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
