use super::Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let check = |m: i32| -> bool {
            let mut cnt = 1;
            let mut s = 0;
            for &x in &nums {
                if s + x <= m {
                    s += x;
                } else {
                    if cnt == k {
                        return false;
                    }
                    cnt += 1;
                    s = x;
                }
            }
            true
        };
        let mut r = nums.iter().sum::<i32>();
        let mut l = (*nums.iter().max().unwrap() - 1).max((r - 1) / k);
        while l + 1 < r {
            let mid = l + (r - l) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_split_array() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
    }
}
