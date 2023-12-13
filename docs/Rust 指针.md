# 指针


deref()方法是将一个智能指针转换为底层数据类型的引用。

```rust
fn main() {
    let a = vec![1, 2, 3];
    a.iter().for_each(|f| println!("{}", f));
    // deref 方法是将一个智能指针转换为底层数据类型的引用。
    let b = a.deref();
    b.iter().for_each(|f| println!("{}", f));
}

```

在上述代码中，Vec是一个智能指针，所以 `a.deref()` 返回的是一个指向底层数据的引用，可以通过这个引用访问到原始的数据。
因此，`a.iter().for_each()` 和` b.iter().for_each()` 打印的内容相同，都是1, 2, 3。

这种用法在许多情况下都非常有用，例如将智能指针转换为对底层数据的可变引用以进行修改。示例如下：

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    {
        let v_mut = v.get_mut(..).unwrap();
        v_mut[0] = 4;
        v_mut[1] = 5;
        v_mut[2] = 6;
    }
    assert_eq!(v, vec![4, 5, 6]);
}

```

在上面的例子中，通过调用deref_mut方法，我们可以获得一个指向底层数据的可变引用，然后就可以对它进行修改。