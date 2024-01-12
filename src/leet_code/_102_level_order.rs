use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn level_order(root: TreeNodePtr) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut stack = vec![];

        if root.is_none() {
            return ret;
        }

        stack.push(root.unwrap());
        while !stack.is_empty() {
            let mut level = vec![];

            for _ in 0..stack.len() {
                let tree = stack.remove(0);
                level.push(tree.borrow_mut().val);

                if tree.borrow_mut().left.is_some() {
                    stack.push(tree.borrow_mut().left.take().unwrap());
                }

                if tree.borrow_mut().right.is_some() {
                    stack.push(tree.borrow_mut().right.take().unwrap());
                }
            }
            ret.push(level);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_level_order() {
        assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::level_order(linked_tree(1, None, None)), vec![vec![1]]);
        assert_eq!(
            Solution::level_order(linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
