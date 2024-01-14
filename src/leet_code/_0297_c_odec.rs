#![allow(dead_code)]

use super::{TreeNode, TreeNodePtr};
use std::cell::RefCell;
use std::rc::Rc;
use std::str;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: TreeNodePtr) -> String {
        fn serialize_helper(root: TreeNodePtr) -> String {
            match root {
                Some(tree) => format!(
                    "{} {} {}",
                    tree.borrow().val,
                    serialize_helper(tree.borrow().left.clone()),
                    serialize_helper(tree.borrow().right.clone())
                ),
                _ => "N".to_string(),
            }
        }

        serialize_helper(root)
    }

    fn deserialize(&self, data: String) -> TreeNodePtr {
        fn deserialize_helper(iter: &mut str::Split<char>) -> TreeNodePtr {
            match iter.next() {
                Some("N") | None => None,
                Some(val) => Some(Rc::new(RefCell::new(TreeNode {
                    val: val.parse().unwrap(),
                    left: deserialize_helper(iter),
                    right: deserialize_helper(iter),
                }))),
            }
        }

        deserialize_helper(&mut data.split(' '))
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::linked_tree;

    use super::Codec;

    #[test]
    fn test_c_odec() {
        let obj = Codec::new();
        let strs = linked_tree(
            1,
            linked_tree(2, None, None),
            linked_tree(3, linked_tree(4, None, None), linked_tree(5, None, None)),
        );

        let data = obj.serialize(strs.clone());
        let ans = obj.deserialize(data);
        assert_eq!(ans, strs.clone());
    }
}
