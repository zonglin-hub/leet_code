use std::cell::RefCell;
use std::rc::Rc;

use leet_code::leet_code::linked_tree;
use leet_code::leet_code::ListNode;
use leet_code::leet_code::TreeNode;
use leet_code::linked_list;

#[test]
fn test_linked_list() {
    assert_eq!(
        linked_list!(1, 2, 3, 4, 5),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        }))
    )
}

#[test]
fn test_linked_tree() {
    assert_eq!(
        linked_tree(1, None, linked_tree(3, None, None)),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))
    )
}
