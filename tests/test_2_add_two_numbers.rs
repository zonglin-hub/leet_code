use leet_code::{create_list, to_vec, Solution};

#[test]
fn test_add_two_numbers_v1() {
    let l1 = None;
    let l2 = None;
    let expected = None;
    assert_eq!(Solution::add_two_numbers_v1(l1, l2), expected);
    let l1 = create_list(vec![2, 4, 3]);
    let l2 = create_list(vec![5, 6, 4]);
    let expected = create_list(vec![7, 0, 8]);
    assert_eq!(
        to_vec(Solution::add_two_numbers_v1(l1, l2)),
        to_vec(expected)
    );
    let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = create_list(vec![9, 9, 9, 9]);
    let expected = create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
    assert_eq!(
        to_vec(Solution::add_two_numbers_v1(l1, l2)),
        to_vec(expected)
    );
}

#[test]
fn test_add_two_numbers_v2() {
    let l1 = None;
    let l2 = None;
    let expected = None;
    assert_eq!(Solution::add_two_numbers_v2(l1, l2), expected);

    let l1 = create_list(vec![2, 4, 3]);
    let l2 = create_list(vec![5, 6, 4]);
    let expected = create_list(vec![7, 0, 8]);
    assert_eq!(
        to_vec(Solution::add_two_numbers_v2(l1, l2)),
        to_vec(expected)
    );

    let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = create_list(vec![9, 9, 9, 9]);
    let expected = create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
    assert_eq!(
        to_vec(Solution::add_two_numbers_v2(l1, l2)),
        to_vec(expected)
    );
}
