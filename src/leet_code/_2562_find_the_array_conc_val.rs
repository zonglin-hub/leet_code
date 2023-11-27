use super::Solution;

impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut ans = 0_i64;
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l <= r {
            if l == r {
                ans += nums[l] as i64;
                break;
            }
            ans += format!("{}{}", nums[l], nums[r]).parse::<i64>().unwrap();
            l += 1;
            r -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_find_the_array_conc_val() {
        assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
        assert_eq!(
            Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]),
            673
        );
    }
}
