use std::iter;

use super::Solution;

const DIRECTION: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn generate_matrix(n: usize) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n]; n];
        let mut cycle = DIRECTION.iter().cycle();
        let (mut i, mut j, mut ns) = (0, -1, 1..);

        for &(move_i, move_j) in iter::once(n)
            .chain((1..n).rev().flat_map(|x| iter::repeat(x).take(2)))
            .flat_map(|x| iter::repeat(cycle.next().unwrap()).take(x))
        {
            i += move_i;
            j += move_j;
            ans[i as usize][j as usize] = ns.next().unwrap();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_generate_matrix() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );

        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
