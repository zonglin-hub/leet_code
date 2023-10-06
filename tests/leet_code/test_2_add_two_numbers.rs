use leet_code::leet_code::{create_list, to_vec, Solution};

#[test]
fn test_add_two_numbers_2_v1() {
    assert_eq!(Solution::add_two_numbers_2_v1(None, None), None);
    assert_eq!(
        to_vec(Solution::add_two_numbers_2_v1(
            create_list(vec![2, 4, 3]),
            create_list(vec![5, 6, 4])
        )),
        to_vec(create_list(vec![7, 0, 8]))
    );
    assert_eq!(
        to_vec(Solution::add_two_numbers_2_v1(
            create_list(vec![9, 9, 9, 9, 9, 9, 9]),
            create_list(vec![9, 9, 9, 9])
        )),
        to_vec(create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]))
    );
}

#[test]
fn test_add_two_numbers_2_v2() {
    assert_eq!(Solution::add_two_numbers_2_v2(None, None), None);
    assert_eq!(
        to_vec(Solution::add_two_numbers_2_v2(
            create_list(vec![2, 4, 3]),
            create_list(vec![5, 6, 4])
        )),
        to_vec(create_list(vec![7, 0, 8]))
    );
    assert_eq!(
        to_vec(Solution::add_two_numbers_2_v2(
            create_list(vec![9, 9, 9, 9, 9, 9, 9]),
            create_list(vec![9, 9, 9, 9])
        )),
        to_vec(create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]))
    );
}

#[test]
fn test_add_strings_415_v1() {
    assert_eq!(
        Solution::add_strings_415_v1("11".to_string(), "123".to_string()),
        "134".to_string()
    );
    assert_eq!(
        Solution::add_strings_415_v1("456".to_string(), "77".to_string()),
        "533".to_string()
    );
    assert_eq!(
        Solution::add_strings_415_v1("0".to_string(), "0".to_string()),
        "0".to_string()
    );
}

#[test]
fn test_add_strings() {
    assert_eq!(
        Solution::add_strings("11".to_string(), "123".to_string()),
        "134".to_string()
    );
    assert_eq!(
        Solution::add_strings("456".to_string(), "77".to_string()),
        "533".to_string()
    );
    assert_eq!(
        Solution::add_strings("0".to_string(), "0".to_string()),
        "0".to_string()
    );
}

#[test]
fn test_and_then() {
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|sq| sq.to_string())
    }

    assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
    assert_eq!(None.and_then(sq_then_to_string), None);

    let arr_2d = [["A0", "A1"], ["B0", "B1"]];

    let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
    assert_eq!(item_0_1, Some(&"A1"));

    let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
    assert_eq!(item_2_0, None);
}
