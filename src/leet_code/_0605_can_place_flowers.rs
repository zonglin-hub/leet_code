//! 种花问题

use super::Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let m = flowerbed.len();
        let mut prev = -1_isize;

        for (i, _) in flowerbed.iter().enumerate().take(m) {
            if flowerbed[i] == 1 {
                if prev < 0 {
                    count += i / 2;
                } else {
                    count += (i - prev as usize - 2) / 2;
                }
                prev = i as isize;
            }
        }

        if prev < 0 {
            count += (m + 1) / 2;
        } else {
            count += (m - prev as usize - 1) / 2;
        }

        count >= n as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_fib_v1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
