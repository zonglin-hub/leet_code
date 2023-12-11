use super::{Solution, TreeNodePtr};

impl Solution {
    // pub fn _min_depth(root: TreeNodePtr) -> i32 {
    //     if root.is_none() {
    //         return 0;
    //     }

    //     let left = root.clone().unwrap().borrow_mut().left.take();
    //     let right = root.unwrap().borrow_mut().right.take();

    //     if left.is_none() && right.is_none() {
    //         return 1;
    //     }

    //     if left.is_none() {
    //         return Self::min_depth(right) + 1;
    //     }

    //     if right.is_none() {
    //         return Self::min_depth(left) + 1;
    //     }

    //     Self::min_depth(left).min(Self::min_depth(right)) + 1
    // }

    pub fn min_depth(root: TreeNodePtr) -> i32 {
        if root.is_none() {
            return 0;
        }

        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.unwrap().borrow_mut().right.take();

        match (left, right) {
            (Some(t), None) | (None, Some(t)) => Self::min_depth(Some(t)) + 1,
            (Some(l), Some(r)) => (1 + Self::min_depth(Some(l))).min(1 + Self::min_depth(Some(r))),
            (None, None) => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_min_depth() {
        assert_eq!(
            Solution::min_depth(linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            2
        );
        assert_eq!(
            Solution::min_depth(linked_tree(
                2,
                None,
                linked_tree(
                    3,
                    None,
                    linked_tree(4, None, linked_tree(5, None, linked_tree(6, None, None)))
                )
            )),
            5
        );
    }
}
