use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_reverse() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
}

#[test]
fn test_reverse_list_206_v1() {
    assert_eq!(
        Solution::reverse_list_206_v1(create_list(vec![1, 2, 3, 4, 5])),
        create_list(vec![5, 4, 3, 2, 1])
    );
    assert_eq!(
        Solution::reverse_list_206_v1(create_list(vec![1, 2])),
        create_list(vec![2, 1])
    );
    assert_eq!(Solution::reverse_list_206_v1(None), None);
    assert_eq!(
        Solution::reverse_list_206_v1(create_list(vec![1, 2, 3, 4, 5])),
        create_list(vec![5, 4, 3, 2, 1])
    );
}

#[test]
fn test_reverse_list_206_v2() {
    assert_eq!(
        Solution::reverse_list_206_v2(create_list(vec![1, 2, 3, 4, 5])),
        create_list(vec![5, 4, 3, 2, 1])
    );
    assert_eq!(
        Solution::reverse_list_206_v2(create_list(vec![1, 2])),
        create_list(vec![2, 1])
    );
    assert_eq!(Solution::reverse_list_206_v2(None), None);
    assert_eq!(
        Solution::reverse_list_206_v2(create_list(vec![1, 2, 3, 4, 5])),
        create_list(vec![5, 4, 3, 2, 1])
    );
}
