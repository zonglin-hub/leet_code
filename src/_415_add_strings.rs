//! 字符串相加
use crate::Solution;

impl Solution {
    pub fn to_int_vec(s: &str) -> Vec<i32> {
        s.bytes().map(|x| (x - b'0') as i32).rev().collect()
    }

    pub fn add_strings(nums1: String, nums2: String) -> String {
        let s1 = Self::to_int_vec(&nums1);
        let s2 = Self::to_int_vec(&nums2);
        let mut carry = 0;
        let mut s3 = vec![];
        let n1 = s1.len();
        let n2 = s2.len();
        let mut i = 0;

        while i < n1 || i < n2 || carry > 0 {
            let mut v = 0;

            if i < n1 {
                v += s1[i];
            }

            if i < n2 {
                v += s2[i];
            }

            v += carry;
            carry = v / 10;
            s3.push(((v % 10) as u8 + b'0') as char);
            i += 1;
        }

        s3.iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_strings() {
        /*
            输入：num1 = "11", num2 = "123"
            输出："134"
        */
        assert_eq!(
            Solution::add_strings("11".to_string(), "123".to_string()),
            "134".to_string()
        );

        /*
            输入：num1 = "456", num2 = "77"
            输出："533"
        */
        assert_eq!(
            Solution::add_strings("456".to_string(), "77".to_string()),
            "533".to_string()
        );

        /*
            输入：num1 = "0", num2 = "0"
            输出："0"
        */
        assert_eq!(
            Solution::add_strings("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }
}
