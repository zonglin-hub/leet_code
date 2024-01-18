use super::Solution;

impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        beans.sort_unstable();
        beans.iter().fold(0, |acc, &x| acc + x as i64)
            - beans
                .iter()
                .enumerate()
                .fold(0, |acc, (i, &val)| acc.max(val as i64 * (beans.len() - i) as i64))
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_removal() {
        assert_eq!(Solution::minimum_removal(vec![4, 1, 6, 5]), 4);
        assert_eq!(Solution::minimum_removal(vec![2, 10, 3, 2]), 7);
    }
}
