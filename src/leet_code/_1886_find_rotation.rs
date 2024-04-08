use super::Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        fn find(mat: &mut Vec<Vec<i32>>, target: &Vec<Vec<i32>>) -> bool {
            let mut temp;
            let n = mat.len();

            for _ in 0..4 {
                for i in 0..n / 2 {
                    for j in 0..(n + 1) / 2 {
                        temp = mat[i][j];
                        mat[i][j] = mat[n - 1 - j][i];
                        mat[n - 1 - j][i] = mat[n - 1 - i][n - 1 - j];
                        mat[n - 1 - i][n - 1 - j] = mat[j][n - 1 - i];
                        mat[j][n - 1 - i] = temp;
                    }
                }

                if mat == target {
                    return true;
                }
            }

            false
        }

        let mut mat = mat;
        find(&mut mat, &target)
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::find_rotation(
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]]
        ));
        assert!(!Solution::find_rotation(
            vec![vec![0, 1], vec![1, 1]],
            vec![vec![1, 0], vec![0, 1]]
        ));
        assert!(Solution::find_rotation(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
        ));
    }
}
