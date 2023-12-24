use super::Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let quotient = n as u32 / 3;
        let remainder = 3_i32;
        match n % 3 {
            0 => remainder.pow(quotient),
            1 => remainder.pow(quotient - 1) * 4,
            _ => remainder.pow(quotient) * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
