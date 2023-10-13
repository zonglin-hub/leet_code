use leet_code::leet_code::Solution;

#[test]
fn test_find_median_sorted_arrays_v2() {
    assert_eq!(
        Solution::find_median_sorted_arrays_v2(vec![1, 3], vec![2]),
        2.00000
    );
    assert_eq!(
        Solution::find_median_sorted_arrays_v2(vec![1, 2], vec![3, 4]),
        2.50000
    );
}

#[test]
fn test_append() {
    let mut vec = vec![1, 7, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    vec.sort();
    assert_eq!(vec, [1, 3, 4, 5, 6, 7]);
    assert_eq!(vec2, []);
}
