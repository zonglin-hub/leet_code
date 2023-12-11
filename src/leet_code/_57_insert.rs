use super::Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(vec![new_interval[0], new_interval[1]]);
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut res = Vec::new();

        let (mut l, mut r) = (intervals[0][0], intervals[0][1]);
        for i in intervals {
            if i[0] > r {
                res.push(vec![l, r]);
                l = i[0];
            }
            r = r.max(i[1]);
        }
        res.push(vec![l, r]);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_insert() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
        assert_eq!(
            Solution::insert(Vec::<Vec<i32>>::new(), vec![5, 7]),
            vec![vec![5, 7]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 3]),
            vec![vec![1, 5]]
        );
        // assert_eq!(
        //     Solution::insert(vec![vec![1, 5]], vec![2, 7]),
        //     vec![vec![1, 7]]
        // );
    }
}
