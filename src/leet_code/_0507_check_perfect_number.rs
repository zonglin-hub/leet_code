use super::Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        let mut sum = 1;
        let mut d = 2;

        while d * d <= num {
            if num % d == 0 {
                sum += d;
                if d * d < num {
                    sum += num / d;
                }
            }
            d += 1;
        }
        sum == num
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_two_sum() {
        assert!(Solution::check_perfect_number(28));
        assert!(!Solution::check_perfect_number(7));
    }
}
