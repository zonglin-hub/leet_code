use super::Solution;

impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let mut ans = 0;
        let bits_len = matrix[0].len();
        let tab = matrix
            .into_iter()
            .map(|a| a.into_iter().enumerate().fold(0, |x, (i, b)| x | (b << i) as u32))
            .collect::<Vec<u32>>();
        for x in 0_u32..1 << bits_len {
            if x.count_ones() != num_select as u32 {
                continue;
            }
            ans = ans.max(tab.iter().fold(0, |cnt, &a| cnt + ((a & x) == a) as i32));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_maximum_rows() {
        assert_eq!(Solution::maximum_rows(vec![vec![1], vec![0]], 1), 2);
        assert_eq!(
            Solution::maximum_rows(
                vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
                2
            ),
            3
        );
    }
}
