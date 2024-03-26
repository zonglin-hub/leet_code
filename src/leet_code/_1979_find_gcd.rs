use super::Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let find_gcd_num = |mut v: (i32, i32)| -> i32 {
            while v.1 != 0 {
                v = (v.1, v.0 % v.1)
            }
            v.0
        };
        find_gcd_num((*nums.iter().max().unwrap(), *nums.iter().min().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
        assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
    }
}
