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

#[cfg(test)]
mod tests {
    use super::*;

    /*
        输入：nums = [1,2,3]
        输出："firstsecondthird"
        解释：
        有三个线程会被异步启动。输入 [1,2,3] 表示线程 A 将会调用 first() 方法，线程 B 将会调用 second() 方法，线程 C 将会调用 third() 方法。正确的输出是 "firstsecondthird"。
    */
    #[test]
    fn test_first_second_third() {
        let foo = Arc::new(Foo::new());
        let nums = vec![1, 2, 3];
        let mut threads = Vec::new();
        for &num in nums.iter() {
            let foo_clone = foo.clone();
            match num {
                1 => {
                    threads.push(std::thread::spawn(move || {
                        foo_clone.first();
                    }));
                }
                2 => {
                    threads.push(std::thread::spawn(move || {
                        foo_clone.second();
                    }));
                }
                3 => {
                    threads.push(std::thread::spawn(move || {
                        foo_clone.third();
                    }));
                }
                _ => {}
            }
        }
        for thread in threads {
            thread.join().expect("");
        }
    }

    /*
        输入：nums = [1,3,2]
        输出："firstsecondthird"
        解释：
        输入 [1,3,2] 表示线程 A 将会调用 first() 方法，线程 B 将会调用 third() 方法，线程 C 将会调用 second() 方法。正确的输出是 "firstsecondthird"。
    */
    #[test]
    fn test_first_third_second() {
        let foo = Arc::new(Foo::new());
        let nums = vec![1, 3, 2];
        let mut threads = Vec::new();
        for &num in nums.iter() {
            let foo_clone = foo.clone();
            match num {
                1 => {
                    threads.push(std::thread::spawn(move || {
                        foo_clone.first();
                    }));
                }
                2 => {
                    threads.push(std::thread::spawn(move || {
                        foo_clone.second();
                    }));
                }
                3 => {
                    threads.push(std::thread::spawn(move || {
                        foo_clone.third();
                    }));
                }
                _ => {}
            }
        }
        for thread in threads {
            thread.join().expect("");
        }
    }
}
