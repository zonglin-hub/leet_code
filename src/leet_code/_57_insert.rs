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
