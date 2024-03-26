use super::{Solution, TreeNodePtr};

impl Solution {
    /// 将二叉树转化为链表
    pub fn flatten(root: &mut TreeNodePtr) {
        if let Some(node) = root {
            let mut n = node.as_ptr();
            unsafe {
                Self::flatten(&mut (*n).left);
                Self::flatten(&mut (*n).right);
                let temp = (*n).right.clone();
                // (*n).right = (*n).left.clone();
                (*n).right.clone_from(&(*n).left);
                (*n).left = None;

                while let Some(nn) = (*n).right.clone() {
                    n = nn.as_ptr();
                }

                (*n).right = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::leet_code::{linked_tree, Solution};
    #[test]
    fn test_flatten() {
        let mut root = linked_tree(
            1,
            linked_tree(2, linked_tree(3, None, None), linked_tree(4, None, None)),
            linked_tree(5, None, linked_tree(6, None, None)),
        );
        Solution::flatten(&mut root);
        assert_eq!(
            root,
            linked_tree(
                1,
                None,
                linked_tree(
                    2,
                    None,
                    linked_tree(
                        3,
                        None,
                        linked_tree(4, None, linked_tree(5, None, linked_tree(6, None, None)))
                    )
                )
            )
        );

        let mut root = None;
        Solution::flatten(&mut root);
        assert_eq!(root, None);

        let mut root = linked_tree(0, None, None);
        Solution::flatten(&mut root);
        assert_eq!(root, linked_tree(0, None, None));
    }
}
