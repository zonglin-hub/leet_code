use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn is_balanced(root: TreeNodePtr) -> bool {
        fn get_height(root: &TreeNodePtr) -> i32 {
            match root {
                Some(x) => {
                    let left_height = get_height(&x.borrow().left);

                    if left_height == -1 {
                        return -1;
                    }

                    let right_height = get_height(&x.borrow().right);

                    if right_height == -1 || (left_height - right_height).abs() > 1 {
                        return -1;
                    }

                    left_height.max(right_height) + 1
                }
                None => 0,
            }
        }

        get_height(&root) != -1
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_is_balanced() {
        assert!(Solution::is_balanced(None));
        assert!(Solution::is_balanced(linked_tree(
            3,
            linked_tree(9, None, None),
            linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
        )));
        assert_eq!(
            Solution::is_balanced(linked_tree(
                1,
                linked_tree(
                    2,
                    linked_tree(3, linked_tree(4, None, None), linked_tree(4, None, None)),
                    linked_tree(3, None, None)
                ),
                linked_tree(2, None, None)
            )),
            false
        );
    }
}
