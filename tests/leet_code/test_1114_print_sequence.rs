use std::sync::Arc;

use leet_code::leet_code::_1114_print_sequence::{Foo, FooTrait};


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
