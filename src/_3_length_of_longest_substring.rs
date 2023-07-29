#![allow(unused)]
struct Solution;

impl Solution {
    /// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
    ///
    /// 无重复字符的最长子串
    ///
    /// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
    ///
    /// 代码的实现思路是使用双指针，在遍历 s 的过程中维护一个滑动窗口，用哈希表来记录字符是否出现过。
    /// 具体来说，用变量 l 记录窗口的左端点，变量 cnt 记录当前窗口中不重复字符的个数，变量 ans 记录最长的不重复子串的长度。
    /// 在遍历 s 的过程中，如果当前字符 c 在哈希表中没有出现过，说明它是一个新字符，cnt 就可以加 1，更新 ans 的值。否则，说明字符 c 出现过，需要将窗口左端点 l 向右移动，使得窗口中不再有字符 c，并相应地更新 cnt 的值。
    /// 最后再将当前字符 c 插入哈希表中。
    /// 总的来说，这段代码的时间复杂度是 $O(n)$，其中 $n$ 是字符串 s 的长度，空间复杂度也是 $O(n)$，因为需要使用哈希表来存储字符的出现情况。
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut ans, mut cnt) = (0, 0);
        let mut map = std::collections::HashMap::new();
        let s = s.chars().collect::<Vec<_>>();
        let mut l = 0;

        s.iter().enumerate().for_each(|(i, c)| {
            match map.get(c) {
                None => {
                    cnt += 1;
                    ans = ans.max(cnt);
                }
                Some(&i) => {
                    for c in &s[l..=i] {
                        map.remove(c);
                    }
                    cnt -= i - l;
                    l = i + 1;
                }
            }
            map.insert(*c, i);
        });
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("ab".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("aba".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbbabcd".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_longest_substring("tmmzuxt".to_string()),
            5
        );
        assert_eq!(
            Solution::length_of_longest_substring("ohomm".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }
}
