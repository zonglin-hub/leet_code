use super::Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        mat.into_iter().enumerate().fold(vec![0, 0], |res, (i, r)| {
            let cnt = r.into_iter().filter(|&x| x == 1).count() as i32;

            if cnt > res[1] {
                vec![i as i32, cnt]
            } else {
                res
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_row_and_maximum_ones() {
        assert_eq!(Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]), vec![0, 1]);
        assert_eq!(Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]), vec![1, 2]);
        assert_eq!(
            Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }
}
