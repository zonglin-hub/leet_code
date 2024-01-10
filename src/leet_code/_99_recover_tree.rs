use super::{Solution, TreeNodePtr};

impl Solution {
    pub fn recover_tree(root: &mut TreeNodePtr) {
        #[inline]
        fn inorder(root: &TreeNodePtr, nums: &mut Vec<i32>) {
            if let Some(node) = root {
                inorder(&node.borrow().left, nums);
                nums.push(node.borrow().val);
                inorder(&node.borrow().right, nums);
            }
        }

        #[inline]
        fn find_two_swapped(nums: &Vec<i32>) -> (i32, i32) {
            let mut index1 = -1;
            let mut index2 = -1;
            for i in 0..nums.len() - 1 {
                if nums[i + 1] < nums[i] {
                    index2 = (i + 1) as i32;
                    if index1 == -1 {
                        index1 = i as i32;
                    } else {
                        break;
                    }
                }
            }
            (nums[index1 as usize], nums[index2 as usize])
        }

        #[inline]
        fn recover(root: &mut TreeNodePtr, mut count: i32, x: i32, y: i32) {
            if let Some(node) = root {
                recover(&mut node.borrow_mut().right, count, x, y);
                if node.borrow().val == x || node.borrow().val == y {
                    node.borrow_mut().val = if node.borrow().val == x { y } else { x };
                    count -= 1;
                    if count == 0 {
                        return;
                    }
                }
                recover(&mut node.borrow_mut().left, count, x, y);
            }
        }

        let mut nums = Vec::new();
        inorder(root, &mut nums);
        let swapped = find_two_swapped(&nums);
        recover(root, 2, swapped.0, swapped.1);
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
