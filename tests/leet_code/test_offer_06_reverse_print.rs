use leet_code::leet_code::{create_list, Solution};

#[test]
fn test_reverse_print_v1() {
    /*
        输入：head = [1,3,2]
        输出：[2,3,1]
    */
    assert_eq!(
        Solution::reverse_print_v1(create_list(vec![1, 3, 2])),
        vec![2, 3, 1]
    );
}
