use std::iter;

use super::Solution;

const DIRECTION: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn generate_matrix(n: usize) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n]; n];
        let mut cycle = DIRECTION.iter().cycle();
        let (mut i, mut j, mut ns) = (0, -1, 1..);
        let iter_x = iter::once(n)
            .chain((1..n).rev().flat_map(|x| iter::repeat(x).take(2)))
            .flat_map(|x| iter::repeat(cycle.next().unwrap()).take(x));

        for &(move_i, move_j) in iter_x {
            i += move_i;
            j += move_j;
            ans[i as usize][j as usize] = ns.next().unwrap();
        }
        ans
    }
}
