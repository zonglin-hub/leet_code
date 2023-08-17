fn main() {
    let mut x = 1;
    x <<= 1; // x *= 2;
    x = (x << 1) + (x << 3); // x *= 10;
    x >>= 1; // x /= 2;
    println!("{}", x);

    let mut a = 5;
    let mut b = 3;
    a ^= b;
    b ^= a;
    a ^= b; // 交换两个值
    println!("{} {}", a, b);
}
