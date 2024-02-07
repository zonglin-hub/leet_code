use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn replace_value_in_tree(root: TreeNodePtr) -> TreeNodePtr {
        fn dfs(node: &TreeNodePtr, count: &mut Vec<i32>, depth: usize) {
            if let Some(node) = node {
                if depth >= count.len() {
                    count.push(0);
                }
                count[depth] += node.borrow().val;
                dfs(&node.borrow().left, count, depth + 1);
                dfs(&node.borrow().right, count, depth + 1);
            }
        }

        fn modify(node: &TreeNodePtr, count: &Vec<i32>, depth: usize) {
            if let Some(node) = node {
                let mut sum = 0;
                let left = node.borrow_mut().left.clone();
                let right = node.borrow_mut().right.clone();

                if let Some(left_node) = &left {
                    sum += left_node.borrow().val;
                }

                if let Some(right_node) = &right {
                    sum += right_node.borrow().val;
                }

                if let Some(left_node) = &left {
                    left_node.borrow_mut().val = count[depth + 1] - sum;
                }

                if let Some(right_node) = &right {
                    right_node.borrow_mut().val = count[depth + 1] - sum;
                }

                modify(&node.borrow_mut().left, count, depth + 1);
                modify(&node.borrow_mut().right, count, depth + 1);
            }
        }

        let mut count = vec![];
        dfs(&root, &mut count, 0);
        modify(&root, &count, 0);

        if let Some(v) = &root {
            v.borrow_mut().val = 0;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::replace_value_in_tree(linked_tree(
                5,
                linked_tree(4, linked_tree(1, None, None), linked_tree(10, None, None)),
                linked_tree(9, None, linked_tree(7, None, None))
            )),
            linked_tree(
                0,
                linked_tree(0, linked_tree(7, None, None), linked_tree(7, None, None)),
                linked_tree(0, None, linked_tree(11, None, None))
            )
        );

        assert_eq!(
            Solution::replace_value_in_tree(linked_tree(
                3,
                linked_tree(1, None, None),
                linked_tree(2, None, None)
            )),
            linked_tree(0, linked_tree(0, None, None), linked_tree(0, None, None))
        );
    }
}
