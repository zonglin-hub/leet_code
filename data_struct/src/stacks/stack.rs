//! 栈 的实现
//！

#![allow(unused)]

#[derive(Debug, Default)]
pub struct Stack<T> {
    // 栈顶
    top: usize,
    // 栈 数据容器
    data: Vec<T>,
}

impl<T> Stack<T> {
    ///  创建一个 空栈，它不需要 参数，返回一个 空栈
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }
}

impl<T> Stack<T> {
    /// 将数据项 val 添加到栈顶，它需要 val 做参数，不返回任何内容
    pub fn push(&mut self, val: T) {
        // 数据 保存在 Vec 末尾
        self.data.push(val);
        self.top += 1;
    }

    /// 从栈 中删除顶部数据项，它不需要 参数，返回 数据项，栈 被修改。
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        // 栈顶 减 1 后再弹出数据
        self.top -= 1;
        self.data.pop()
    }

    /// 从栈返回顶部数据项，但不会删除它，不需要参数，不修改栈
    pub fn peek(&self) -> Option<&T> {
        // 数据不能 移动，只能 返回引用
        if self.top == 0 {
            return None;
        }
        self.data.last()
    }

    /// 测试栈是否为空，不需要参数，返回布尔值。
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    ///  返回栈中数据项的数量，不需要参数，返回一个 usize 型整数
    pub fn size(&self) -> usize {
        // 栈顶 恰好就是 栈中元素 个数
        self.top
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(4);

        // println!("top {:?}, size {}", s.peek().expect(""), s.size());
        // println!("pop {:?}, size {}", s.pop().expect(""), s.size());
        // println!("is_empty:{}, stack:{:?}", s.is_empty(), s);

        assert_eq!(s.peek().expect(""), &4);
        assert_eq!(s.size(), 3);
        assert_eq!(s.pop().expect(""), 4);
        assert!(!s.is_empty());
    }
}
