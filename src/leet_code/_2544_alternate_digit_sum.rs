use super::Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let (mut ans, mut sign) = (0, -1);
        while n != 0 {
            sign = -sign;
            ans += n % 10 * sign;
            n /= 10;
        }
        sign * ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_alternate_digit_sum() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
        assert_eq!(Solution::alternate_digit_sum(111), 1);
        assert_eq!(Solution::alternate_digit_sum(886996), 0);
    }
}
