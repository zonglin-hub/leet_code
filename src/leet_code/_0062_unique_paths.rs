use super::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let u = (m + n - 2) as i64;
        let d = (m - 1).min(n - 1) as i64;
        let mut res = 1;

        for (x, y) in (u - d + 1..=u).zip(1..=d) {
            res = res * x / y;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
        assert_eq!(Solution::unique_paths(51, 9), 1916797311);
    }
}
