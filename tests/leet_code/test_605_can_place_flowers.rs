use leet_code::leet_code::Solution;

#[test]
fn test_fib_v1() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
}

#[test]
fn test_take() {
    let a = [1, 2, 3];

    // 删除与给定范围对应的子切片，并返回对它的引用。
    let mut iter = a.iter().take(2);

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}
