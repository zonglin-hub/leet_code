use super::Solution;

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut n = (needed_apples as f64 / 4.0).cbrt() as i64;
        if 2 * n * (n + 1) * (2 * n + 1) < needed_apples {
            n += 1;
        }
        8 * n
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_perimeter() {
        assert_eq!(Solution::minimum_perimeter(1), 8);
        assert_eq!(Solution::minimum_perimeter(13), 16);
        assert_eq!(Solution::minimum_perimeter(1000000000), 5040);
    }
}
