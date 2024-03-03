use std::collections::VecDeque;

#[derive(Default)]
pub struct MyStack {
    pub data: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self { data: VecDeque::new() }
    }

    /// 将元素压入栈顶
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_code::leet_code::_0225_my_stack::MyStack;
    ///
    /// let mut my_stack = MyStack::new();
    /// my_stack.push(1);
    /// ```
    pub fn push(&mut self, x: i32) {
        self.data.push_back(x)
    }

    /// 移除并返回栈顶元素
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_code::leet_code::_0225_my_stack::MyStack;
    ///
    /// let mut my_stack = MyStack::new();
    /// my_stack.push(1);
    /// my_stack.push(2);
    /// assert_eq!(2, my_stack.pop());
    /// ```
    pub fn pop(&mut self) -> i32 {
        self.data.pop_back().unwrap()
    }

    /// 返回栈顶元素
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_code::leet_code::_0225_my_stack::MyStack;
    ///
    /// let mut my_stack = MyStack::new();
    /// my_stack.push(1);
    /// my_stack.push(2);
    /// assert_eq!(2, my_stack.top());
    /// ```
    pub fn top(&self) -> i32 {
        *self.data.back().unwrap()
    }

    /// 如果栈是空的返回 true 否则返回 false
    ///
    /// # Examples
    /// ```
    /// use leet_code::leet_code::_0225_my_stack::MyStack;
    ///
    /// let mut my_stack = MyStack::new();
    /// assert!(my_stack.empty());
    /// my_stack.push(1);
    /// assert!(!my_stack.empty());
    /// ```
    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {

    use super::MyStack;

    #[test]
    fn test_my_stack() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(2, obj.top());
        assert_eq!(2, obj.pop());
        assert!(!obj.empty());
    }
}
