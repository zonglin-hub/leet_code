use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn zigzag_level_order(root: TreeNodePtr) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if root.is_none() {
            return res;
        }

        let mut node_queue = vec![root];
        let mut flag = false;

        while !node_queue.is_empty() {
            let mut values = vec![];
            let mut new_queue = vec![];

            for node in node_queue.into_iter().flatten() {
                values.push(node.borrow().val);
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();

                if left.is_some() {
                    new_queue.push(left);
                }

                if right.is_some() {
                    new_queue.push(right);
                }
            }

            if flag {
                values.reverse();
            }

            res.push(values);
            flag = !flag;
            node_queue = new_queue;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_zigzag_level_order() {
        assert_eq!(Solution::zigzag_level_order(None), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::zigzag_level_order(linked_tree(1, None, None)), vec![vec![1]]);
        assert_eq!(
            Solution::zigzag_level_order(linked_tree(
                3,
                linked_tree(9, None, None),
                linked_tree(20, linked_tree(15, None, None), linked_tree(7, None, None))
            )),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}
