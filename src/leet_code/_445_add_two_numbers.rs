// //! 两数相加 II

// use super::ListNodePtr;
// use super::Solution;

// impl Solution {
//     pub fn add_two_numbers_445(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
//         Solution::reverse_list(Solution::add_two_numbers(
//             Solution::reverse_list(l1),
//             Solution::reverse_list(l2),
//         ))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::ListNode;
//     use crate::leet_code::Solution;
//     use crate::linked_list;

//     #[test]
//     fn test_add_two_numbers_445() {
//         assert_eq!(
//             Solution::add_two_numbers_445(linked_list!(7, 2, 4, 3), linked_list!(5, 6, 4)),
//             linked_list!(7, 8, 0, 7)
//         );
//         assert_eq!(
//             Solution::add_two_numbers_445(linked_list!(2, 4, 3), linked_list!(5, 6, 4)),
//             linked_list!(8, 0, 7)
//         );
//         assert_eq!(
//             Solution::add_two_numbers_445(linked_list!(0), linked_list!(0)),
//             linked_list!(0)
//         );
//     }
// }
