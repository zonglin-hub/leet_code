use leet_code::leet_code::to_node;
use leet_code::leet_code::ListMaker;
use leet_code::leet_code::ListNode;
use leet_code::list;

#[test]
fn test_() {
    assert_eq!(
        list!(1, 2, 3, 4, 5),
        to_node(1, to_node(2, to_node(3, to_node(4, to_node(5, None)))))
    )
}
