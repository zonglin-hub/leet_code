use std::collections::HashMap;

use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn find_mode(root: TreeNodePtr) -> Vec<i32> {
        #[inline]
        fn pre_order(root: &TreeNodePtr, map: &mut HashMap<i32, usize>) {
            if let Some(node) = root {
                let tree = node.borrow_mut();
                *map.entry(tree.val).or_insert(0) += 1;
                pre_order(&tree.left, map);
                pre_order(&tree.right, map);
            }
        }

        let mut map = HashMap::new();
        pre_order(&root, &mut map);

        // let mut max_count = 0;
        // for &i in map.values() {
        //     if i > max_count {
        //         max_count = i;
        //     }
        // }
        // let max_count = map.values().max().cloned().unwrap_or(0);
        // let mut acc = Vec::new();
        // for (&val, &count) in map.iter() {
        //     if count == max_count {
        //         acc.push(val)
        //     }
        // }
        // acc..
        map.iter()
            .filter(|&(_, &count)| count == map.values().max().cloned().unwrap_or(0))
            .map(|(&val, _)| val)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_find_mode() {
        assert_eq!(
            Solution::find_mode(linked_tree(
                1,
                None,
                linked_tree(2, linked_tree(2, None, None), None)
            )),
            vec![2]
        );
        assert_eq!(Solution::find_mode(linked_tree(0, None, None)), vec![0]);
    }
}
