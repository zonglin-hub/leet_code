use super::Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
    }
}
