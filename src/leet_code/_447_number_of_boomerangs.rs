use super::Solution;

use std::collections::HashMap;

impl Solution {
    /// 枚举 + 哈希表(力扣官方题解)
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for p in &points {
            let mut cnt = HashMap::new();
            for q in &points {
                let dis = (p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]);
                cnt.insert(dis, cnt.get(&dis).unwrap_or(&0) + 1);
            }

            for m in cnt.values() {
                res += m * (m - 1);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_number_of_boomerangs() {
        assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
            2
        );
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            2
        );
    }
}
