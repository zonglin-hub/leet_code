//! 串联所有单词的子串

use crate::Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // 定义一个HashMap，用于存储字符串和其出现的次数
        use std::collections::HashMap;

        // 将字符串转换为字节数组
        let s = s.as_bytes();
        // 获取字符串长度
        let n = s.len();
        // 获取单词的长度
        let k = words[0].len();
        // 如果字符串长度大于单词的长度，则返回空的结果
        if k > n {
            return vec![];
        }

        // 初始化一个空的Vec
        let mut out = vec![];
        // 初始化一个HashMap，用于存储字符串和其出现的次数
        let mut map =
            words
                .iter()
                .fold(HashMap::<&[u8], (usize, usize)>::new(), |mut map, word| {
                    map.entry(word.as_bytes()).or_default().0 += 1;
                    map
                });

        // 初始化一个变量，用于标记是否重置map
        let mut map_is_reset = true;

        // 定义一个重置函数
        macro_rules! reset {
            () => {
                // 如果map_is_reset为false，则重置map
                if !map_is_reset {
                    map_is_reset = true;
                    // 遍历map，将每个元素的1减1
                    map.iter_mut().for_each(|(_, value)| value.1 = 0);
                }
            };
        }

        // 遍历字符串，计算出现次数
        for i in 0..k {
            // 重置map
            reset!();

            // 定义两个变量，分别表示当前位置和下一位置
            let (mut lo, mut hi) = (i, i);

            // 当hi小于字符串长度时，结束循环
            while hi <= n - k {
                // 如果map中存在当前位置的字符串，则将其出现次数加1
                match map.get_mut(&s[hi..hi + k]) {
                    None => {
                        hi += k;
                        lo = hi;
                        reset!();
                    }
                    // 如果map中不存在当前位置的字符串，则将其出现次数设置为1，并将hi加k
                    Some(hi_value) => {
                        hi_value.1 += 1;
                        hi += k;
                        map_is_reset = false;

                        // 如果hi_value.1大于hi_value.0，则循环
                        if hi_value.1 > hi_value.0 {
                            loop {
                                // 将lo加k，将lo_value.1减1
                                let lo_value = map.get_mut(&s[lo..lo + k]).expect("");
                                lo += k;
                                lo_value.1 -= 1;

                                // 如果lo_value.0等于lo_value.1，则退出循环
                                if lo_value.0 == lo_value.1 {
                                    break;
                                }
                            }
                        }
                    }
                }

                if hi - lo == words.len() * k {
                    out.push(lo as i32);
                    map.get_mut(&s[lo..lo + k]).expect("").1 -= 1;
                    lo += k;
                }
            }
        }

        out
    }
}
