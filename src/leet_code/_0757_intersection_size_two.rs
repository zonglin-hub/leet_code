use super::Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
        intervals
            .iter()
            .fold((0, -1, -1), |(cnt, second_end, first_end), interval| {
                if interval[0] > first_end {
                    (cnt + 2, interval[1] - 1, interval[1])
                } else if interval[0] > second_end {
                    (
                        cnt + 1,
                        if first_end == interval[1] { first_end - 1 } else { first_end },
                        interval[1],
                    )
                } else {
                    (cnt, second_end, first_end)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_intersection_size_two() {
        assert_eq!(Solution::intersection_size_two(vec![vec![1, 3], vec![3, 7], vec![8, 9]]), 5);
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
            3
        );
        assert_eq!(
            Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
            5
        );
    }
}
