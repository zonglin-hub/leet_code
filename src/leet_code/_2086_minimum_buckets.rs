//! 从房屋收集雨水需要的最少水桶数
//!

use super::Solution;

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut ans = 0;
        let mut bucket = false;
        let mut house = false;
        let mut buckets = vec![false; hamsters.len()];
        for (i, c) in hamsters.chars().enumerate() {
            match c {
                'H' => {
                    if i > 1 && buckets[i - 1] {
                        house = false;
                        bucket = false;
                        continue;
                    }
                    if house {
                        if bucket {
                            bucket = false;
                            ans += 1;
                        } else {
                            return -1;
                        }
                    }
                    house = true;
                }
                '.' => {
                    if house {
                        house = false;
                        ans += 1;
                        bucket = false;
                        buckets[i] = true;
                    } else {
                        bucket = true;
                    }
                }
                _ => {}
            }
        }

        if bucket && house {
            ans += 1;
        } else if house {
            return -1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_minimum_buckets() {
        assert_eq!(Solution::minimum_buckets("H..H".to_string()), 2);
        assert_eq!(Solution::minimum_buckets(".H.H.".to_string()), 1);
        assert_eq!(Solution::minimum_buckets(".HHH.".to_string()), -1);
        assert_eq!(Solution::minimum_buckets("H".to_string()), -1);
        assert_eq!(Solution::minimum_buckets(".".to_string()), 0);
    }
}
