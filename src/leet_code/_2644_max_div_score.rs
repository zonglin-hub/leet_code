use super::Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut ans = 0;

        for &div in &divisors {
            let mut tmp = 0;
            for num in &nums {
                if num % div == 0 {
                    tmp += 1;
                }
            }

            if tmp > cnt || tmp == cnt && div < ans {
                cnt = tmp;
                ans = div;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]), 3);
    }
}
