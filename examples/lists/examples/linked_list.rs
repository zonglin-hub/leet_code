#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
