use super::Solution;

impl Solution {
    #[allow(clippy::int_plus_one)]
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut left = 0;
        let mut right = mat.len() - 1;
        let index_of_max = |a: &Vec<i32>| (0..a.len()).max_by_key(|x| a[*x]).unwrap();

        while left <= right {
            let i = (left + right) / 2;
            let j = index_of_max(&mat[i]);

            if i as i32 - 1 >= 0 && mat[i][j] < mat[i - 1][j] {
                right = i - 1;
                continue;
            }

            if i + 1 < mat.len() && mat[i][j] < mat[i + 1][j] {
                left = i + 1;
                continue;
            }

            return vec![i as i32, j as i32];
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_peak_grid() {
        assert_eq!(Solution::find_peak_grid(vec![vec![1, 4], vec![3, 2]]), vec![0, 1]);
        assert_eq!(
            Solution::find_peak_grid(vec![vec![10, 20, 15], vec![21, 30, 14], vec![7, 16, 32]]),
            vec![1, 1]
        );
        assert_eq!(
            Solution::find_peak_grid(vec![vec![10, 40, 41], vec![21, 30, 14], vec![7, 16, 32]]),
            vec![0, 2]
        );
        assert_eq!(
            Solution::find_peak_grid(vec![
                vec![16, 10, 7, 4, 19, 25, 40, 27, 16],
                vec![11, 21, 37, 47, 21, 13, 49, 11, 32],
                vec![33, 30, 32, 20, 32, 2, 6, 26, 33],
                vec![30, 16, 14, 38, 23, 4, 19, 32, 39],
                vec![42, 44, 31, 19, 35, 1, 7, 28, 9],
                vec![17, 47, 24, 49, 26, 24, 3, 26, 4]
            ]),
            vec![5, 3]
        );
    }
}
