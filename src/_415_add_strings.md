# code_415


## x - b'0' 什么意思

```rust
let s1: Vec<i32> = nums1.bytes().map(|x| (x - b'0') as i32).rev().collect();
```

在 Rust 中，字母和数字都被视为 ASCII 码，ASCII 码值用整数表示。

例如，字符 '0' 的 ASCII 码值为 48，而字符 '1' 的 ASCII 码值为 49，以此类推。

在这段代码中，表达式 x - b'0' 就是将 ASCII 码值转换为对应的数字值，

例如字符 '0' 被转换为整数 0，字符 '1' 被转换为整数 1，以此类推。这样操作之后就可以得到一个数字类型的向量 s1，其中包含了字符串 nums1 表示的每个数字的数字值。


## 参考文档:

- https://www.asciim.cn/