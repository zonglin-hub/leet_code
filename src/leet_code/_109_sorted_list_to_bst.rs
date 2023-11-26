use std::{cell::RefCell, rc::Rc};

use super::{ListNodePtr, Solution, TreeNode, TreeNodePtr};

impl Solution {
    /// 这个函数`sorted_list_to_bst`的目的是将一个有序链表转化为一个平衡的二叉搜索树（BST）。函数接受一个链表的头节点作为参数，并返回一个二叉搜索树的根节点。
    ///
    /// 函数内部定义了一个名为`build`的辅助函数，该函数接受一个整数数组作为参数，并返回一个二叉搜索树的根节点。这个辅助函数使用递归的方式来构建二叉搜索树。
    ///
    /// 1. 如果输入的数组是空的，则返回`None`，表示没有节点。
    /// 2. 计算数组的中点索引`mid`。这里使用整数除法，所以结果是一个整数。
    /// 3. 创建一个新的二叉树节点，节点的值为数组中点处的值`nodes[mid]`。
    /// 4. 递归调用`build`函数来构建左子树，传入数组的前半部分`nodes[..mid]`作为参数。
    /// 5. 递归调用`build`函数来构建右子树，传入数组的后半部分`nodes[mid + 1..]`作为参数。
    /// 6. 返回创建的二叉树节点的根节点。
    ///
    /// 在主函数部分：
    ///
    /// 1. 创建一个空的整数数组`nodes`，用于存储链表中的元素。
    /// 2. 通过迭代遍历链表，将链表中的元素依次存储到`nodes`数组中。
    /// 3. 调用辅助函数`build`，传入`nodes`数组作为参数，开始构建二叉搜索树。
    /// 4. 返回构建完成的二叉搜索树的根节点。
    ///
    /// 总结：该函数通过遍历链表将链表元素存储到数组中，然后使用递归的方式构建平衡的二叉搜索树。
    pub fn sorted_list_to_bst(head: ListNodePtr) -> TreeNodePtr {
        fn build(nodes: &[i32]) -> TreeNodePtr {
            if nodes.is_empty() {
                return None;
            }

            let mid = nodes.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nodes[mid],
                left: build(&nodes[..mid]),
                right: build(&nodes[mid + 1..]),
            })))
        }

        let mut nodes = Vec::new();
        let mut head = &head;
        while let Some(node) = head.as_ref() {
            nodes.push(node.val);
            head = &node.next;
        }

        build(&nodes)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::leet_code::{create_list, Solution, TreeNode};

    #[test]
    fn test_sorted_list_to_bst() {
        assert_eq!(
            Solution::sorted_list_to_bst(create_list(vec![-10, -3, 0, 5, 9])),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -10,
                        left: None,
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))
        );

        assert_eq!(Solution::sorted_list_to_bst(None), None);
    }
}
