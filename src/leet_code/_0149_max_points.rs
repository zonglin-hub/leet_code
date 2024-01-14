use std::collections::HashMap;

use super::Solution;

#[inline]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        let t = b;
        b = a % b;
        a = t;
        if b == 0 {
            return a;
        }
    }
}

fn proj_rep(p1: &[i32], p2: &[i32]) -> (i32, i32, i32) {
    let (mut a, mut b) = (p1[1] - p2[1], p2[0] - p1[0]);
    let mut c = -a * p1[0] - b * p1[1];
    match a.signum() {
        -1 => {
            a = -a;
            b = -b;
            c = -c;
        }
        0 => return (0, 1, c / b),
        _ => (),
    }
    let d = gcd(b.abs(), a);
    (a / d, b / d, c / d)
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let (mut ret, mut record) = (1, HashMap::new());
        for j in 1..points.len() {
            for i in 0..j {
                record.entry(proj_rep(&points[i], &points[j])).and_modify(|v| *v += 1).or_insert(1);
            }
            ret = ret.max(1 + record.drain().map(|(_, v)| v).max().unwrap());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_max_points() {
        assert_eq!(Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
    }
}
