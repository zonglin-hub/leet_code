#![allow(unused)]

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn create_linked_list(values: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for &val in values.iter().rev() {
        let node = ListNode {
            val,
            next: head.take(),
        };

        head = Some(Box::new(node));
    }

    head
}

// macro_rules! linked_list {
//     // 匹配空的列表
//     () => {
//         None
//     };
//     // 匹配非空的列表
//     ($( $val:expr ),*) => {{
//         let mut head = None;
//         let mut tail = &mut head;

//         $(
//             let node = ListNode {val: $val, next: None};
//             *tail = Some(Box::new(node));
//             tail = &mut tail.as_mut().expect("").next;
//         )*

//         head.and_then(|mut node| {
//             let mut prev = None;
//             while let Some(mut current) = node.next.take() {
//                 node.next = prev.take();
//                 prev = Some(node);
//                 node = current;
//             }
//             node.next = prev.take();
//             Some(node)
//         })
//     }};
// }

macro_rules! linked_list {
    // 匹配到空的列表
    () => {
        None
    };
    // 匹配到非空的列表
    ( $( $val:expr ),* ) => {{
        let mut head = None;

        $(
            let node = ListNode {
                val: $val,
                next: head.take(),
            };

            head = Some(Box::new(node));
        )*

        head
    }};
}

fn main() {
    let values = vec![1, 1, 2, 3, 4, 4, 5, 6];
    let head = create_linked_list(&values);
    let list = linked_list![1, 1, 2, 3, 4, 4, 5, 6];

    println!("{:?}", list);
    println!("{:?}", head);
}
