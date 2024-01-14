use super::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
