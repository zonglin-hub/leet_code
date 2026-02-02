use super::Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter()
            .map(|arr| arr.iter().map(|e| if *e < 0 { 1 } else { 0 }).sum::<i32>())
            .sum::<i32>()
    }

    pub fn count_negatives_v1(grid: Vec<Vec<i32>>) -> i32 {
        let mut nums = 0;
        for row in grid {
            for value in row {
                if value < 0 {
                    nums += 1
                }
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_count_negatives() {
        assert_eq!(
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }

    #[test]
    fn test_count_negatives_v1() {
        assert_eq!(
            Solution::count_negatives_v1(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(Solution::count_negatives_v1(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
