use super::Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut x = 0;
        let mut y = matrix[0].len() as i32 - 1;

        while x < matrix.len() && y >= 0 {
            match matrix[x][y as usize].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => y -= 1,
                Ordering::Less => x += 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_search_matrix_ii() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::search_matrix_ii(matrix.clone(), 5));
        assert!(!Solution::search_matrix_ii(matrix.clone(), 20));
    }
}
