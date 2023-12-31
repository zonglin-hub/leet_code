use super::Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let mut amount = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let year = date[..4].parse::<i32>().unwrap();
        if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
            amount[1] += 1;
        }

        let month = date[5..7].parse::<usize>().unwrap();
        amount[..month - 1].iter().sum::<i32>() + date[8..].parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_day_of_year() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    }
}
