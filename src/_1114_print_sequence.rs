//! 按序打印

use std::sync::{Arc, Condvar, Mutex};

pub trait FooTrait {
    fn first(&self);
    fn second(&self);
    fn third(&self);
}

#[derive(Debug, Default)]
pub struct Foo {
    cv12: Arc<Condvar>,
    cv23: Arc<Condvar>,
    order: Arc<Mutex<i32>>,
}

impl FooTrait for Foo {
    // 第一个函数，用于模拟第一个线程
    fn first(&self) {
        // 将order的值设置为1
        let mut order = self.order.lock().expect("");
        assert_eq!(*order, 1);
        print!("first");
        *order = 2;
        // 唤醒cv12的线程
        self.cv12.notify_one();
    }

    // 第二个函数，用于模拟第二个线程
    fn second(&self) {
        // 将order的值设置为2
        let mut order = self.order.lock().expect("");
        while *order != 2 && *order != 3 {
            // 等待cv12的线程
            order = self.cv12.wait(order).expect("");
        }
        print!("second");
        *order = 3;
        // 唤醒cv23的线程
        self.cv23.notify_one();
    }

    // 第三个函数，用于模拟第三个线程
    fn third(&self) {
        // 将order的值设置为3
        let mut order = self.order.lock().expect("");
        while *order != 3 {
            // 等待cv23的线程
            order = self.cv23.wait(order).expect("");
        }
        print!("third");
    }
}

impl Foo {
    pub fn new() -> Self {
        Foo {
            cv12: Arc::new(Condvar::new()),
            cv23: Arc::new(Condvar::new()),
            order: Arc::new(Mutex::new(1)),
        }
    }
}
