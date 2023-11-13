use super::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.into_iter().flatten().any(|f| f == target)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let expected = Solution::search_matrix(matrix, 3);
        assert!(expected);

        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let expected = Solution::search_matrix(matrix, 13);
        assert_eq!(expected, false);
    }

    #[test]
    fn test_flatten() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let v1 = vec![1, 3, 5, 7, 10, 11, 16, 20, 23, 30, 34, 60];
        let expected = matrix.into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(expected, v1);
    }
}
