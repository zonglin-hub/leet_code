//! 全部开花的最早一天

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

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_buckets() {
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
            9
        );
        assert_eq!(
            Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
            9
        );
        assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
    }

    #[test]
    fn test_sort_unstable_by() {
        let mut v = [5, 4, 1, 3, 2];
        v.sort_unstable_by(|a, b| {
            println!("a: {} b: {}", a, b);
            a.cmp(b)
        });
        assert!(v == [1, 2, 3, 4, 5]);

        // 反向排序
        v.sort_unstable_by(|a, b| b.cmp(a));
        assert!(v == [5, 4, 3, 2, 1]);
    }
}
