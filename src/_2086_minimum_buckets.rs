//! 从房屋收集雨水需要的最少水桶数
//!

use crate::types::base_type::Solution;

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut ans = 0;
        let mut bucket = false;
        let mut house = false;
        let mut buckets = vec![false; hamsters.len()];

        for (i, c) in hamsters.chars().enumerate() {
            // 如果当前字符为'H'
            match c {
                'H' => {
                    // 那么需要判断是否前一个字符也为'H'，如果是，则将house和bucket设置为false。
                    if i > 1 && buckets[i - 1] {
                        house = false;
                        bucket = false;
                        continue;
                    }

                    // 然后判断house是否为true，如果是，则需要判断bucket是否为true，如果是，则将bucket设置为false，并增加ans的值。
                    if house {
                        if bucket {
                            bucket = false;
                            ans += 1;
                        } else {
                            return -1;
                        }
                    }
                    // 最后将house设置为true。
                    house = true;
                }
                // 如果当前字符为'.'，那么需要判断house是否为true，如果是，则将bucket设置为true，并增加ans的值，同时将buckets[i]设置为true。
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
                // 其他字符，如'A'、'B'等，则直接跳过。
                _ => {}
            }
        }

        // 在遍历结束后，需要判断bucket和house是否都为true，如果是，则将ans加1。然后判断ans是否大于0，如果是，则返回ans，否则返回-1。
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
    use super::*;

    #[test]
    fn test_minimum_buckets() {
        // 输入：street = "H..H"
        // 输出：2
        // 解释：我们可以在下标为 1 和 2 处放水桶。"H..H" -> "HBBH"（'B' 表示放置水桶）。下标为 0 处的房屋右边有水桶，下标为 3 处的房屋左边有水桶。所以每个房屋旁边都至少有一个水桶收集雨水。
        assert_eq!(Solution::minimum_buckets("H..H".to_string()), 2);

        // 输入：street = ".H.H."
        // 输出：1
        // 解释：我们可以在下标为 2 处放置一个水桶。".H.H." -> ".HBH."（'B' 表示放置水桶）。下标为 1 处的房屋右边有水桶，下标为 3 处的房屋左边有水桶。所以每个房屋旁边都至少有一个水桶收集雨水。
        assert_eq!(Solution::minimum_buckets(".H.H.".to_string()), 1);

        // 输入：street = ".HHH."
        // 输出：-1
        // 解释：没有空位可以放置水桶收集下标为 2 处的雨水。所以没有办法收集所有房屋的雨水。
        assert_eq!(Solution::minimum_buckets(".HHH.".to_string()), -1);

        // 输入：street = "H"
        // 输出：-1
        // 解释：没有空位放置水桶。所以没有办法收集所有房屋的雨水。
        assert_eq!(Solution::minimum_buckets("H".to_string()), -1);

        // 输入：street = "."
        // 输出：0
        // 解释：没有房屋需要收集雨水。所以需要 0 个水桶。
        assert_eq!(Solution::minimum_buckets(".".to_string()), 0);
    }
}
