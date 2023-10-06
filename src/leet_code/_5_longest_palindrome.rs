//! 最长回文子串
//! 相似题型：5 | 9 | 234 | 2472

use super::{ListNodePtr, Solution};

impl Solution {
    /// 给你一个字符串 s，找到 s 中最长的回文子串。
    /// 如果字符串的反序与原始字符串相同，则该字符串称为回文字符串。
    pub fn longest_palindrome_5_v1(s: String) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut windows = s.len();
        let mut head = 0;
        while head != sv.len() {
            if head + windows > sv.len() {
                windows -= 1;
                head = 0;
                continue;
            }
            if Self::is_palindrome(&sv[head..head + windows]) {
                return sv[head..head + windows].iter().collect::<String>();
            }
            head += 1
        }
        "".to_string()
    }

    fn is_palindrome(v: &[char]) -> bool {
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

impl Solution {
    pub fn is_palindrome_9_v2(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }
}

impl Solution {
    pub fn is_palindrome_234_v1(head: ListNodePtr) -> bool {
        let (mut val, mut node) = (vec![], &head);
        // loop {
        //     let node_box = match node {
        //         Some(x) => x,
        //         None => break,
        //     };
        //     val.push(node_box.val);
        //     node = &node_box.next;
        // }

        while let Some(node_box) = node {
            val.push(node_box.val);
            node = &node_box.next;
        }
        let val_rev = val.clone();
        val.reverse();
        val == val_rev
    }
}

impl Solution {
    /// 将字符串转换为字节数组，记录字符串长度n。
    /// 定义一个长度为n+1的数组v，`v[i]` 表示前缀字符串s[0..i]中可以构成的最大回文串数。
    /// 遍历所有的子串，对于每个子串s[l..r]：-> 如果子串长度小于k，不做处理。 -> 如果子串可以构成回文串，更新 `v[r+1]` 为 `v[l]+1`。 -> 否则，左右扩展子串，继续判断是否可以构成回文串。
    /// 最终返回最大回文串数。
    pub fn max_palindromes_2472(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut v = vec![0; n + 1];

        for i in 0..2 * n - 1 {
            let mut l = i / 2;
            let mut r = l + i % 2;

            v[l + 1] = std::cmp::max(v[l + 1], v[l]);

            while r < n && s[l] == s[r] {
                if r - l + 1 >= k as usize {
                    v[r + 1] = std::cmp::max(v[r + 1], v[l] + 1);
                    break;
                }

                if l == 0 {
                    break;
                }

                l -= 1;
                r += 1;
            }
        }

        v[n]
    }
}

impl Solution {
    fn valid(s: &[u8]) -> bool {
        let n = s.len();
        s.iter().rev().take(n).eq(s.iter().take(n))
    }

    pub fn valid_palindrome_680(s: String) -> bool {
        let s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            if s[i] != s[j] {
                /*
                    `i + 1..=j` 是一个 Rust 中的元组表示法，用来表示一个范围。在这个例子中，它表示从 `i + 1` 到 `j` 的所有整数，包括 `i + 1` 和 `j`。
                    例如，如果 `i = 0` 且 `j = 5`，那么 `0 + 1..=5` 就表示了从 `1` 到 `5` 的所有整数，即 `1, 2, 3, 4, 5`。
                */
                return Self::valid(&s[i + 1..=j]) || Self::valid(&s[i..=j - 1]);
            }

            i += 1;
            j -= 1;
        }

        true
    }
}
