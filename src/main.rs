// fn main() {
//     println!("Hello, world!");
// }

use std::{cell::RefCell, rc::Rc};

// 定义二叉树节点
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 将数组转成二叉树
pub fn array_to_tree(root: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_empty() {
        return None;
    }
    let root_node = Rc::new(RefCell::new(TreeNode::new(root[0])));
    let mut queue = vec![Rc::clone(&root_node)];
    let mut i = 1;
    while i < root.len() {
        let node = queue.remove(0);
        if root[i] != -1 {
            let left = Rc::new(RefCell::new(TreeNode::new(root[i])));
            node.borrow_mut().left = Some(Rc::clone(&left));
            queue.push(left);
        }
        i += 1;
        if i < root.len() && root[i] != -1 {
            let right = Rc::new(RefCell::new(TreeNode::new(root[i])));
            node.borrow_mut().right = Some(Rc::clone(&right));
            queue.push(right);
        }
        i += 1;
    }
    Some(root_node)
}

// 测试
fn main() {
    let root = vec![3, 2, 3, -1, 3, -1, 1];
    let tree = array_to_tree(root);
    println!("Binary Tree: {:?}", tree);
}
