use super::Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut high_bit = 0;
        for i in 1..=30 {
            if num <= 1 << i {
                break;
            }
            high_bit = i
        }

        let mask = if high_bit == 30 { 0x7fffffff } else { (1 << (high_bit + 1)) - 1 };
        num ^ mask
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement(1), 0);
    }
}
