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
