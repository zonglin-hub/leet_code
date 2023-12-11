use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn max_path_sum(root: TreeNodePtr) -> i32 {
        fn max_gain(root: &TreeNodePtr, max_sum: &mut i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let left_gain = 0.max(max_gain(&node.borrow().left, max_sum));
                    let right_gain = 0.max(max_gain(&node.borrow().right, max_sum));
                    *max_sum = (*max_sum).max(node.borrow().val + left_gain + right_gain);
                    node.borrow().val + left_gain.max(right_gain)
                }
            }
        }

        let mut max_sum = i32::MIN;
        max_gain(&root, &mut max_sum);
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_single_number() {
        assert_eq!(
            Solution::max_path_sum(linked_tree(
                1,
                linked_tree(2, None, None),
                linked_tree(3, None, None)
            )),
            6
        );
        assert_eq!(
            Solution::max_path_sum(linked_tree(
                -10,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            42
        );
    }
}
