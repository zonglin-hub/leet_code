use leet_code::leet_code::Solution;

#[test]
fn test_is_valid_v1() {
    assert!(Solution::is_valid_v1(String::from("()")));
    assert!(Solution::is_valid_v1(String::from("[]")));
    assert!(Solution::is_valid_v1(String::from("{}")));
    assert!(Solution::is_valid_v1(String::from("()[]{}")));
    assert!(Solution::is_valid_v1(String::from("({[]})")));
    assert_eq!(Solution::is_valid_v1(String::from("(]")), false);
}

#[test]
fn test_vec_drain() {
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);

    // 全范围清除 vector，就像 `clear()` 一样
    v.drain(..);
    assert_eq!(v, &[]);
}

#[test]
fn test_fold() {
    let a = [1, 4, 3];

    // 数组所有元素的总和
    // acc 累加器
    // x 迭代器遍历的数据
    let sum = a.iter().fold(0, |acc, x| {
        println!("acc: {}", acc);
        println!("x: {}", x);
        acc + x
    });

    assert_eq!(sum, 8);
}
