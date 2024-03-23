use super::Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1)
            .map(|i| match (0 == i % 3, 0 == i % 5) {
                (true, true) => "FizzBuzz".to_owned(),
                (true, false) => "Fizz".to_owned(),
                (false, true) => "Buzz".to_owned(),
                _ => i.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1".to_owned(), "2".to_owned(), "Fizz".to_owned()]);
        assert_eq!(
            Solution::fizz_buzz(5),
            vec![
                "1".to_owned(),
                "2".to_owned(),
                "Fizz".to_owned(),
                "4".to_owned(),
                "Buzz".to_owned()
            ]
        );
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1".to_owned(),
                "2".to_owned(),
                "Fizz".to_owned(),
                "4".to_owned(),
                "Buzz".to_owned(),
                "Fizz".to_owned(),
                "7".to_owned(),
                "8".to_owned(),
                "Fizz".to_owned(),
                "Buzz".to_owned(),
                "11".to_owned(),
                "Fizz".to_owned(),
                "13".to_owned(),
                "14".to_owned(),
                "FizzBuzz".to_owned()
            ]
        );
    }
}
