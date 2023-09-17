use leet_code::leet_code::{create_list, to_vec, Solution};

#[test]
fn test_add_two_numbers_v1() {
    assert_eq!(Solution::add_two_numbers_v1(None, None), None);
    assert_eq!(
        to_vec(Solution::add_two_numbers_v1(
            create_list(vec![2, 4, 3]),
            create_list(vec![5, 6, 4])
        )),
        to_vec(create_list(vec![7, 0, 8]))
    );
    assert_eq!(
        to_vec(Solution::add_two_numbers_v1(
            create_list(vec![9, 9, 9, 9, 9, 9, 9]),
            create_list(vec![9, 9, 9, 9])
        )),
        to_vec(create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]))
    );
}

#[test]
fn test_add_two_numbers_v2() {
    assert_eq!(Solution::add_two_numbers_v2(None, None), None);
    assert_eq!(
        to_vec(Solution::add_two_numbers_v2(
            create_list(vec![2, 4, 3]),
            create_list(vec![5, 6, 4])
        )),
        to_vec(create_list(vec![7, 0, 8]))
    );
    assert_eq!(
        to_vec(Solution::add_two_numbers_v2(
            create_list(vec![9, 9, 9, 9, 9, 9, 9]),
            create_list(vec![9, 9, 9, 9])
        )),
        to_vec(create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]))
    );
}
