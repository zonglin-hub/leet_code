use super::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.into_iter().fold(vec![vec![]], |v, n| {
            v.into_iter()
                .flat_map(|f| [[vec![n], f.clone()].concat(), f])
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_subsets() {
        let out = vec![
            vec![3, 2, 1],
            vec![2, 1],
            vec![3, 1],
            vec![1],
            vec![3, 2],
            vec![2],
            vec![3],
            vec![],
        ];
        assert_eq!(Solution::subsets(vec![1, 2, 3]), out);

        let out = vec![vec![0], vec![]];
        assert_eq!(Solution::subsets(vec![0]), out);
    }
}
