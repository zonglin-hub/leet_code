use super::Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut ans = vec![];
        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);

        intervals.iter().skip(1).for_each(|f| {
            if f[0] > end {
                ans.push(vec![start, end]);
                start = f[0];
            }
            end = end.max(f[1]);
        });
        ans.push(vec![start, end]);
        ans
    }
}
