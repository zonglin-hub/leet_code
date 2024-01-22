fn main() {
    // 这是一个位运算符号 << 的示例，表示将一个二进制数向左移动指定的位数，类似于十进制数的乘以2的n次方。例如：
    // 0 左移 1 位，结果为 0，因为二进制中的 0 向左移动 1 位仍然是 0。
    // 1 左移 1 位，结果为 2，因为二进制中的 1 向左移动 1 位变成了 10，而 10 表示的十进制数就是 2。
    // 2 左移 1 位，结果为 4，因为二进制中的 10 向左移动 1 位变成了 100，而 100 表示的十进制数就是 4。
    // assert_eq!(0 << 1, 0);
    // assert_eq!(1 << 1, 2);
    // assert_eq!(2 << 1, 4);

    // << 是一个左移操作符，将左操作数的位向左移动右操作数指定的位数，并用零填充空位。
    //
    // 例如:
    //
    // 1 << 0 将整数 1 的位向左移动 0 位，这不会改变其值，因为向左移动 0 位没有影响。
    // 1 << 1 将整数 1 的位向左移动 1 位，这会将最低有效位设置为 0，第二个最低有效位设置为 1，结果二进制值为 10，即十进制值 2。
    // 类似地，1 << 2 将整数 1 的位向左移动 2 位，结果为 100，即十进制值 4，以此类推。
    //
    // 这些位移操作有一个规律，即左移操作符 << 将数值的二进制位向左移动 n 位，相当于将原来的值乘以 $2^n$。
    //
    // 因此，$1 << n$ 将等于 1 乘以 $2^n$。
    //
    // 例如:
    //
    // $1 << 0$ 左移 0 位，相当于 $1 \times 2^0 = 1$；
    // $1 << 1$ 左移 1 位，相当于 $1 \times 2^1 = 2$；
    // $1 << 2$ 左移 2 位，相当于 $1 \times 2^2 = 4$；
    // $1 << 3$ 左移 3 位，相当于 $1 \times 2^3 = 8$；
    // $1 << 4$ 左移 4 位，相当于 $1 \times 2^4 = 16$。
    // 因此，可以用左移操作符来快速计算某个数的二进制表示中的 2 的指数幂。
    // assert_eq!(1 << 0, 1);
    // assert_eq!(1 << 1, 2);
    // assert_eq!(1 << 2, 4);
    // assert_eq!(1 << 3, 8);
    // assert_eq!(1 << 4, 16);

    // 这是一个位运算符号 >> 的示例，表示将一个二进制数向右移动指定的位数，类似于十进制数的除以2的n次方。例如：
    // 0 右移 1 位，结果为 0，因为二进制中的 0 向右移动 1 位仍然是 0。
    // 1 右移 1 位，结果为 0，因为二进制中的 1 向右移动 1 位变成了 0，而 0 表示的十进制数就是 0。
    // 2 右移 1 位，结果为 1，因为二进制中的 10 向右移动 1 位变成了 1，而 1 表示的十进制数就是 1。
    // 3 右移 1 位，结果为 1，因为二进制中的 11 向右移动 1 位变成了 1，而 1 表示的十进制数就是 1。
    // 4 右移 1 位，结果为 2，因为二进制中的 100 向右移动 1 位变成了 10，而 10 表示的十进制数就是 2。
    // assert_eq!(0 >> 1, 0);
    // assert_eq!(1 >> 1, 0);
    // assert_eq!(2 >> 1, 1);
    // assert_eq!(3 >> 1, 1);
    // assert_eq!(4 >> 1, 2);

    let mut x = 1;
    x <<= 1; // x *= 2;
    x = (x << 1) + (x << 3); // x *= 10;
    x >>= 1; // x /= 2;
    assert_eq!(x, 10);

    let mut a = 5;
    let mut b = 3;
    // a ^= b;
    // b ^= a;
    // a ^= b; // 交换两个值
    std::mem::swap(&mut a, &mut b); // 交换两个值
    assert_eq!(a, 3);
    assert_eq!(b, 5);
}
