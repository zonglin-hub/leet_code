use super::{Solution, TreeNodePtr};

impl Solution {
    /// 这段代码实现了一个将二叉树转化为链表的函数`flatten`。它接受一个二叉树的根节点指针作为输入，并将二叉树转化为一个链表。它的实现思路如下：
    ///
    /// 1. 它使用一个循环来遍历二叉树的所有节点。在每次循环中，它将当前节点的左子节点和右子节点作为参数，递归地调用`Self::flatten`函数，将左子节点和右子节点也转化为链表。
    ///
    /// 2. 它使用一个变量`n`来存储当前节点的指针。在每次循环中，它使用`as_ptr`方法将当前节点转换为指针，并将其存储在`n`中。
    ///
    /// 3. 它使用一个`unsafe`块来获取当前节点的左子节点和右子节点的指针。在`unsafe`块中，它使用`(*n).left`和`(*n).right`来获取左子节点和右子节点的指针。
    ///
    /// 4. 它使用一个变量`temp`来存储当前节点的右子节点。在每次循环中，它将当前节点的右子节点存储在`temp`中，并将当前节点的右子节点设置为当前节点的左子节点。这样，就将当前节点的左子节点和右子节点连接起来，形成了一个链表。
    ///
    /// 5. 它使用一个循环来处理当前节点的右子节点。在每次循环中，它将当前节点的右子节点存储在`nn`中，并将`n`向前移动一步。这样，就可以将当前节点的右子节点添加到链表的末尾。
    ///
    /// 6. 最后，它将当前节点的右子节点设置为`temp`，这样就完成了将二叉树转化为链表的操作。
    ///
    /// 通过这种方式，函数`flatten`就可以将二叉树转化为链表。
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
