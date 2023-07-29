#![allow(unused)]
struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    /// 反转链表
    ///这段代码实现了单链表的反转。它的输入参数是一个 Option<Box<ListNode>> 类型的头结点，表示链表的头部。这里使用了 Option 类型和 Box 类型，是为了能够处理空指针的情况。
    /// 在函数中，定义了两个变量 res 和 node，用于存储反转后的链表和原链表。变量 res 初始值为 None，表示反转后的链表为空；变量 node 初始值为输入参数 head，表示原链表的头结点。
    /// 接下来进入循环，使用 while let 语法判断 node 是否是 Some(mut x)，即判断原链表是否还有节点。如果有节点，则执行循环体内的代码块。
    /// 循环体内，首先获取 x 的下一个节点，即 node = x.next.take()，将 node 更新为原链表的下一个节点。接着将 x.next 设为反转后的链表 res，即 x.next = res.take()，这里的 take() 方法会将 res 中的值取出来，并将 res 置为 None，避免出现两个链表共享同一个节点的情况。最后将 x 设为反转后的链表 res，即 res = Some(x)，将反转后的链表头更新为当前节点。
    /// 循环结束后，返回 res，即反转后的链表头，完成单链表的反转。
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut res, mut node) = (None, head);
        while let Some(mut x) = node {
            node = x.next.take();
            x.next = res.take();
            res = Some(x);
        }
        res
    }
}
