fn main() {
    // fn call_with_one<F>(func: F) -> usize
    // where
    //     F: Fn(usize) -> usize,
    // {
    //     func(1)
    // }

    // let double = |x| x * 2;
    // assert_eq!(call_with_one(double), 2);

    let a = |y| y * 2;

    fn call<F>(f: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        // |y| y * 2;
        // let a ||.. 等同 fn a(1){..}
        f(2)
    }
    assert_eq!(call(a), 4);
}
