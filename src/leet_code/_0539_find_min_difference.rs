use super::Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        const TIME: i32 = 1440;

        if time_points.len() > TIME as usize {
            return 0;
        }

        let mut cache = time_points
            .into_iter()
            .map(|t| t[0..2].parse::<i32>().unwrap() * 60 + t[3..].parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        cache.sort_unstable();
        let n = cache.len();

        let mut min = i32::MAX;
        (1..n).for_each(|i| min = std::cmp::min(min, cache[i] - cache[i - 1]));
        std::cmp::min(min, cache[0] + TIME - cache[n - 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_min_difference() {
        assert_eq!(Solution::find_min_difference(vec!["23:59".to_owned(), "00:00".to_owned()]), 1);
        assert_eq!(
            Solution::find_min_difference(vec![
                "00:00".to_owned(),
                "23:59".to_owned(),
                "00:00".to_owned()
            ]),
            0
        );
    }
}
