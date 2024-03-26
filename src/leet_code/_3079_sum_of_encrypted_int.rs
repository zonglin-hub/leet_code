use super::Solution;

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for mut x in nums.into_iter() {
            let mut mx = 0;
            let mut base = 0;
            while x > 0 {
                mx = std::cmp::max(mx, x % 10);
                base = base * 10 + 1; 
                x /= 10;
            }
            ans += mx * base;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_sum_of_encrypted_int() {
        assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}
