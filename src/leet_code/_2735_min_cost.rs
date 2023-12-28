use super::Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut s = (0..n).map(|i| i as i64 * x as i64).collect::<Vec<i64>>();

        for i in 0..n {
            let mut m = nums[i];

            for j in i..(n + i) {
                m = m.min(nums[j % n]);
                s[j - i] += m as i64;
            }
        }
        *s.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_cost() {
        assert_eq!(Solution::min_cost(vec![20, 1, 15], 5), 13);
        assert_eq!(Solution::min_cost(vec![1, 2, 3], 4), 6);
    }
}
