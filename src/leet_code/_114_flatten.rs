use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn flatten(root: &mut TreeNodePtr) {
        if let Some(node) = root {
            let mut n = node.as_ptr();
            unsafe {
                Self::flatten(&mut (*n).left);
                Self::flatten(&mut (*n).right);
                let temp = (*n).right.clone();
                (*n).right = (*n).left.clone();
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
    use std::{cell::RefCell, rc::Rc};

    use crate::leet_code::{Solution, TreeNode};

    #[test]
    fn test_flatten() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        Solution::flatten(&mut root);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 5,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 6,
                                    left: None,
                                    right: None,
                                }))),
                            }))),
                        }))),
                    }))),
                }))),
            })))
        );

        let mut root = None;
        Solution::flatten(&mut root);
        assert_eq!(root, None);

        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        Solution::flatten(&mut root);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None
            })))
        );
    }
}
