use leet_code::leet_code::Solution;

#[test]
fn test_find_subarrays() {
    assert!(Solution::find_subarrays(vec![4, 2, 4]));
    assert_eq!(Solution::find_subarrays(vec![1, 2, 3, 4, 5]), false);
    assert!(Solution::find_subarrays(vec![0, 0, 0]));
}

#[test]
fn test_find_subarrays_v2() {
    assert!(Solution::find_subarrays_v2(vec![4, 2, 4]));
    assert_eq!(Solution::find_subarrays_v2(vec![1, 2, 3, 4, 5]), false);
    assert!(Solution::find_subarrays_v2(vec![0, 0, 0]));
}

#[test]
fn test_windows() {
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &['r', 'u']);
    assert_eq!(iter.next().unwrap(), &['u', 's']);
    assert_eq!(iter.next().unwrap(), &['s', 't']);
    assert!(iter.next().is_none());
}

#[test]
fn test_map() {
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| 2 * x);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
}
