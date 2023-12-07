// #![allow(unused)]

// use std::{
//     cell::{Ref, RefCell},
//     future::IntoFuture,
//     io::SeekFrom,
//     rc::Rc,
// };

// type ListNodePtr = Option<Rc<RefCell<ListNode>>>;

// struct ListNode {
//     val: i32,
//     prev: ListNodePtr,
//     next: ListNodePtr,
// }

// impl ListNode {
//     fn new(val: i32) -> Self {
//         Self {
//             val,
//             prev: None,
//             next: None,
//         }
//     }
// }

// struct MyLinkedList {
//     head: ListNodePtr,
//     size: i32,
// }

// impl MyLinkedList {
//     fn new() -> Self {
//         let mut head = Rc::new(RefCell::new(ListNode::new(1)));
//         head.borrow_mut().prev = Some(head.clone());
//         head.borrow_mut().next = Some(head.clone());

//         Self {
//             head: Some(head),
//             size: 0,
//         }
//     }

//     fn find_node(&self, index: i32) -> ListNodePtr {
//         if self.size == 0 {
//             return self.head.as_ref().unwrap().borrow().next.clone();
//         }

//         if self.size == index {
//             return self.head.as_ref().unwrap().borrow().prev.clone();
//         }

//         let mut curr = self.head.as_ref().unwrap().borrow().next.clone();
//         for _ in 0..index {
//             let node = curr.as_ref().unwrap().borrow().next.clone();
//             curr = node;
//         }

//         curr
//     }

//     fn get(&self, index: i32) -> i32 {
//         if index < 0 || index >= self.size {
//             return -1;
//         }

//         self.find_node(index).as_ref().unwrap().borrow().val
//     }

//     fn add_at_head(&mut self, val: i32) {
//         let next = self.head.as_ref().unwrap().borrow().next.clone();
//         let mut curr = Rc::new(RefCell::new(ListNode {
//             val,
//             prev: self.head.clone(),
//             next: next.clone(),
//         }));
//         next.clone().as_ref().unwrap().borrow_mut().prev = Some(curr.clone());
//         self.head.as_ref().unwrap().borrow_mut().next = Some(curr.clone());
//         self.size += 1;
//     }

//     fn add_at_tail(&mut self, val: i32) {
//         let mut prev = self.head.as_ref().unwrap().borrow().prev.clone();
//         let mut curr = Rc::new(RefCell::new(ListNode {
//             val,
//             prev: prev.clone(),
//             next: self.head.clone(),
//         }));
//         prev.clone().as_ref().unwrap().borrow_mut().next = Some(curr.clone());
//         self.head.as_ref().unwrap().borrow_mut().prev = Some(curr.clone());
//         self.size += 1;
//     }

//     fn add_at_index(&mut self, index: i32, val: i32) {
//         if index > self.size {
//             return;
//         }

//         if index < 0 {
//             return self.add_at_head(val);
//         }

//         if index == self.size {
//             return self.add_at_tail(val);
//         }

//         let curr = self.find_node(index);
//         let node = Rc::new(RefCell::new(ListNode {
//             val,
//             prev: curr.as_ref().unwrap().borrow().prev.clone(),
//             next: curr.clone(),
//         }));

//         curr.as_ref()
//             .unwrap()
//             .borrow_mut()
//             .prev
//             .as_ref()
//             .unwrap()
//             .borrow_mut()
//             .next = Some(node.clone());

//         curr.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
//         self.size += 1;
//     }

//     fn delete_at_index(&mut self, index: i32) {
//         if self.size <= 0 || index < 0 || index >= self.size {
//             return;
//         }

//         if self.size == 1 {
//             self.head.as_ref().unwrap().borrow_mut().prev = self.head.clone();
//             self.head.as_ref().unwrap().borrow_mut().next = self.head.clone();
//             self.size -= 1;
//         }

//         let curr = self.find_node(index);
//         let (prev, next) = (
//             curr.as_ref().unwrap().borrow().prev.clone(),
//             curr.as_ref().unwrap().borrow().next.clone(),
//         );
//         prev.clone().as_ref().unwrap().borrow_mut().next = next.clone();
//         next.clone().as_ref().unwrap().borrow_mut().prev = prev.clone();
//         self.size -= 1;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::MyLinkedList;

//     #[test]
//     fn test_my_linked_list() {
//         let mut obj = MyLinkedList::new();
//         obj.add_at_head(1);
//         obj.add_at_tail(3);
//         obj.add_at_index(1, 2); // 链表变为 1->2->3
//         assert_eq!(obj.get(1), 2); // 返回 2

//         obj.delete_at_index(1); // 现在，链表变为 1->3
//         assert_eq!(obj.get(1), 3); // 返回 3
//     }
// }
