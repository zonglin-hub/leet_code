/// '0 & 1' 实际上是执行 '0000 & 0001'，结果是 '0000'，即 0。
///
/// '0000 & 0001' 是在进行二进制位运算，表示将两个二进制数进行按位与操作。
/// 按位与运算符(&)会对两个操作数的每一位执行与操作，即两个位都为1时，结果为1，否则为0。
/// '0000 & 0001' 可以解释为：
///```
/// // '3 & 4' 实际上是执行 '0011 & 0100'，结果是 '0000'，即 0。
/// // 二进制: 0011
/// //        0100
/// //        ----
/// //        0000
/// assert_eq!(3 & 4, 0);
/// ```
pub fn and() {
    use std::ops::BitAnd;

    #[derive(Debug, PartialEq)]
    struct Scalar(bool);

    impl BitAnd for Scalar {
        type Output = Self;

        // rhs 是表达式 `a & b` 的 "right-hand side"
        fn bitand(self, rhs: Self) -> Self::Output {
            Self(self.0 & rhs.0)
        }
    }

    assert_eq!(Scalar(true) & Scalar(true), Scalar(true));
    assert_eq!(Scalar(true) & Scalar(false), Scalar(false));
    assert_eq!(Scalar(false) & Scalar(true), Scalar(false));
    assert_eq!(Scalar(false) & Scalar(false), Scalar(false));
}
