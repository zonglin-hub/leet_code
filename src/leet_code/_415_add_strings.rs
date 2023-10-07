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
