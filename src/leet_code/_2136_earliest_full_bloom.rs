use super::Solution;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut days = 0;
        let mut id = (0..grow_time.len()).collect::<Vec<usize>>();
        id.sort_unstable_by(|&a, &b| grow_time[b].cmp(&grow_time[a]));
        for &i in &id {
            days += plant_time[i];
            ans = ans.max(days + grow_time[i]);
        }
        ans
    }
}
