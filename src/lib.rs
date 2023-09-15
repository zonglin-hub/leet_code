//! 使用 Rust 解决 Leetcode 相关问题。
//!
//! 先写测试模块，以测试驱动完成 Rust Code。
//!

pub mod leet_code;

use std::{cell::RefCell, rc::Rc};

/// leet code 算法解题
pub struct Solution;

/// 树结构
#[derive(Debug, PartialEq, Eq)]
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

/// 链表结构
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // 用于标记一个函数或方法在内部实现时可以被优化掉，从而减少代码量。
    // 当一个函数或方法被标记为#[inline]时，Rust编译器会在内部将其替换为等效的机器代码，从而减少调用这个函数的开销。
    // 需要注意的是，#[inline]属性应该仅用于优化性能，而不是用于改变函数的行为或它的可见性。
    // 在大多数情况下，Rust会自动内联函数调用，因此您不需要显式地使用#[inline]属性。
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// 简化套娃
pub fn create_tree_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub fn create_list_node(nums: i32) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode {
        val: nums,
        next: None,
    }))
}

/// 创建链表
pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.is_empty() {
        return None;
    }

    // 创建一个头节点，并将其赋值给head
    let mut head = create_list_node(nums[0]);
    // 将head赋值给p
    let mut p = head.as_mut();
    // 遍历nums数组，将每一个元素赋值给ListNode
    for num in nums.iter().skip(1) {
        let node = create_list_node(*num);
        // 将ListNode赋值给p的下一个节点
        p.as_mut().expect("").next = node;
        // 将p的下一个节点赋值给p
        p = p.expect("").next.as_mut();
    }
    // 返回head
    head
}
// fn create_linked_list(values: &Vec<i32>) -> Option<Box<ListNode>> {
//     let mut head = None;
//     for &val in values.iter().rev() {
//         let node = ListNode {
//             val,
//             next: head.take(),
//         };
//         head = Some(Box::new(node));
//     }
//     head
// }

/// 用于测试数组乱序情况
///
/// assertion failed: `(left == right)`
///  left: `[[0, 1], [3, 3], [1, 0]]`,
///  right: `[[0, 1], [1, 0], [3, 3]]`
pub fn expected_sort(queens: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut expected = queens.iter().map(|e| e.to_vec()).collect::<Vec<Vec<i32>>>();
    expected.sort();
    expected
}

/// 类型转换
///
/// let m = vec![[0, 1], [1, 0], [3, 3]]
///
/// 生成算法函数把 m 转成  
///
/// vec![vec![0, 1], vec![1, 0], vec![3, 3]]
///
pub fn expected_sort_vec(queens: Vec<[i32; 2]>) -> Vec<Vec<i32>> {
    // let m = vec![[0, 1], [1, 0], [3, 3]];
    queens
        .iter()
        .map(|&x| x.to_vec())
        .collect::<Vec<Vec<i32>>>()
}

pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    // 创建一个空的数组，用于存放节点的值
    let mut res = vec![];
    // 将head赋值给p
    let mut p = head;
    // 当p存在时，将其值存入res数组
    while let Some(node) = p {
        res.push(node.val);
        // 将p的下一个节点赋值给p
        p = node.next;
    }
    // 返回res数组
    res
}

pub fn to_int_vec(s: &str) -> Vec<i32> {
    s.bytes().map(|x| (x - b'0') as i32).rev().collect()
}
