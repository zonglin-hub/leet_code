//! 常用类型模块

use std::{cell::RefCell, rc::Rc};

pub struct Solution;

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
