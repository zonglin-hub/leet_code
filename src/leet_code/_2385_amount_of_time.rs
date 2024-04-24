use super::{Solution, TreeNode, TreeNodePtr};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

impl Solution {
    pub fn amount_of_time(root: TreeNodePtr, start: i32) -> i32 {
        let mut graph = HashMap::new();
        let mut visited = HashSet::new();

        fn dfs(node_opt: Option<&Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(node_ref) = node_opt {
                let node = node_ref.borrow();
                if let Some(ref left) = &node.left {
                    graph.entry(node.val).or_default().push(left.borrow().val);
                    graph.entry(left.borrow().val).or_default().push(node.val);
                    dfs(Some(left), graph);
                }
                if let Some(ref right) = &node.right {
                    graph.entry(node.val).or_default().push(right.borrow().val);
                    graph.entry(right.borrow().val).or_default().push(node.val);
                    dfs(Some(right), graph);
                }
            }
        }

        if let Some(node) = root.as_ref() {
            dfs(Some(node), &mut graph);
        }

        let mut q = VecDeque::new();
        let mut time = 0;
        q.push_back((start, 0));
        visited.insert(start);
        while let Some((node_val, curr_time)) = q.pop_front() {
            time = curr_time;
            if let Some(children) = graph.get(&node_val) {
                for &child_val in children {
                    if !visited.contains(&child_val) {
                        q.push_back((child_val, time + 1));
                        visited.insert(child_val);
                    }
                }
            }
        }
        time
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::amount_of_time(
                linked_tree(
                    1,
                    linked_tree(
                        5,
                        None,
                        linked_tree(4, linked_tree(9, None, None), linked_tree(2, None, None))
                    ),
                    linked_tree(3, linked_tree(10, None, None), linked_tree(6, None, None))
                ),
                3
            ),
            4
        );
        assert_eq!(Solution::amount_of_time(linked_tree(1, None, None), 1), 0);
    }
}
