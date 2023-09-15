# 解题思路


这道题其实是一道典型的 DFS（深度优先搜索）的题目。动态规划的思路也能实现，但是复杂度较高。下面让我详细解释一下 DFS 的思路。
题目要求生成有效的括号组合，那么我们从最简单的情况考虑，例如生成一对括号的有效组合，这个时候只有一种情况，就是 "()"。
如果我们继续往下加括号，假设加一对括号，那么新的有效组合就可以通过在原有的组合基础上加上一对括号产生。
例如一个有效组合是 "()"，那么在它的左右两侧分别加上一对括号，则新的两个有效组合分别为 "()()" 和 "(())"。
再继续往下加括号，可以发现，新的有效组合其实就可以通过在原有的组合基础上加上一对括号产生。因此，这道题的解法就是 DFS，以当前字符串为基础继续添加括号并验证其合法性，直到添加完 n 对括号。
在 DFS 的过程中，我们需要记录剩余的左右括号数量。
为了保证每添加一对括号，产生的新组合都是唯一的，我们需要限制添加的顺序，即我们可以在剩余的左右括号数量中，只在剩余左括号数量大于剩余右括号数量时，添加右括号。


# Code 优化


```rust
# pub struct Solution;
# impl Solution {
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut cur = "".to_string();
    Self::dfs(n, n, &mut cur, &mut res);
    res
}

fn dfs(l: i32, r: i32, cur: &mut String, res: &mut Vec<String>) {
    if l == 0 && r == 0 {
        res.push(cur.chars().collect())
    } else {
        if l > 0 {
            cur.push('(');
            Self::dfs(l - 1, r, cur, res);
            cur.pop();
        }
        if r > l {
            cur.push(')');
            Self::dfs(l, r - 1, cur, res);
            cur.pop();
        }
    }
}
# }
```

目前的实现中，每次都是将一个字符拼接到字符串的末尾，然后再在递归回退时将其从末尾删除。这样的操作在字符串长度比较短时还是可以的，但是当字符串比较长时，就会变得低效，因为字符串的拼接和删除操作都是 `O(n)` 的。
可以使用 `StringBuilder` 或 `Vec<char>` 等可变字符串来优化这个过程。在每次拼接和删除时，只需要将字符添加到 `StringBuilder` 或 `Vec<char>` 的末尾或删除末尾元素即可，这样就可以避免每次都需要重新构建字符串对象的开销。最后只需要将 `StringBuilder` 或 `Vec<char>` 转换成字符串即可。


```rust
# pub struct Solution;
# impl Solution {
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    let mut cur = Vec::new();
    Self::dfs(n, n, &mut cur, &mut res);
    res
}

fn dfs(l: i32, r: i32, cur: &mut Vec<char>, res: &mut Vec<String>) {
    if l == 0 && r == 0 {
        res.push(cur.iter().collect())
    } else {
        if l > 0 {
            cur.push('(');
            Self::dfs(l - 1, r, cur, res);
            cur.pop();
        }
        if r > l {
            cur.push(')');
            Self::dfs(l, r - 1, cur, res);
            cur.pop();
        }
    }
}
# }
```