//! 字符串相加

use super::{to_int_vec, Solution};

impl Solution {
    pub fn add_strings_415_v1(nums1: String, nums2: String) -> String {
        let s1 = to_int_vec(&nums1);
        let s2 = to_int_vec(&nums2);
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
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_add_strings_415_v1() {
        assert_eq!(
            Solution::add_strings_415_v1("11".to_string(), "123".to_string()),
            "134".to_string()
        );
        assert_eq!(
            Solution::add_strings_415_v1("456".to_string(), "77".to_string()),
            "533".to_string()
        );
        assert_eq!(
            Solution::add_strings_415_v1("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }
}
