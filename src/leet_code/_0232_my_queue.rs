#[derive(Default)]
pub struct MyQueue {
    ins: Vec<i32>,
    out: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self { ins: Vec::new(), out: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.ins.push(x)
    }

    pub fn pop(&mut self) -> i32 {
        self.peek();
        self.out.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        if self.out.is_empty() && !self.ins.is_empty() {
            self.out.push(self.ins.pop().unwrap())
        }
        self.out.last().cloned().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.ins.is_empty() && self.out.is_empty()
    }
}
