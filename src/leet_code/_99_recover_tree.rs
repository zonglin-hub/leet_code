use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn recover_tree(root: &mut TreeNodePtr) {
        fn inorder_traverse<F>(root: &TreeNodePtr, consumer: &mut F)
        where
            F: FnMut(i32),
        {
            if let Some(node) = root {
                inorder_traverse(&node.borrow().left, consumer);
                consumer(root.as_ref().unwrap().borrow().val);
                inorder_traverse(&node.borrow().right, consumer);
            }
        }

        fn inorder(root: TreeNodePtr) -> Vec<i32> {
            let mut res = vec![];
            inorder_traverse(&root, &mut (|x| res.push(x)));
            res
        }

        fn recove(node: &TreeNodePtr, count: &mut i32, x: i32, y: i32) {
            if let Some(n) = node {
                if n.borrow().val == x || n.borrow().val == y {
                    *count -= 1;

                    if *count == 0 {
                        return;
                    }

                    if n.borrow().val == x {
                        n.borrow_mut().val = y;
                    } else {
                        n.borrow_mut().val = x;
                    }
                }
                recove(&n.borrow().right, count, x, y);
                recove(&n.borrow().left, count, x, y);
            }
        }

        fn find_2swap(nums: &[i32]) -> (i32, i32) {
            let n = nums.len();
            let mut x = -1;
            let mut y = -1;

            for i in 0..(n - 1) {
                if nums[i + 1] < nums[i] {
                    y = nums[i + 1];

                    if x == -1 {
                        x = nums[i];
                    } else {
                        break;
                    }
                }
            }
            (x, y)
        }

        let ret = inorder(root.to_owned());
        let (x, y) = find_2swap(&ret);
        let mut cnt = 3;
        recove(root, &mut cnt, x, y);
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::{linked_tree, Solution};

    #[test]
    fn test_recover_tree() {
        let mut root = linked_tree(1, linked_tree(3, None, linked_tree(2, None, None)), None);
        Solution::recover_tree(&mut root);
        assert_eq!(
            root,
            linked_tree(3, linked_tree(1, None, linked_tree(2, None, None)), None)
        );

        let mut root = linked_tree(
            3,
            linked_tree(1, None, None),
            linked_tree(4, linked_tree(2, None, None), None),
        );
        Solution::recover_tree(&mut root);
        assert_eq!(
            root,
            linked_tree(
                2,
                linked_tree(1, None, None),
                linked_tree(4, linked_tree(3, None, None), None)
            )
        );
    }
}
