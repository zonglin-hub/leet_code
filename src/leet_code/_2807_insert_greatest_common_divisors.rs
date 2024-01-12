use crate::leet_code::ListNode;

use super::{ListNodePtr, Solution};

impl Solution {
    pub fn insert_greatest_common_divisors(head: ListNodePtr) -> ListNodePtr {
        #[inline]
        fn gcd(x: i32, y: i32) -> i32 {
            if y == 0 {
                x
            } else {
                gcd(y, x % y)
            }
        }

        #[inline]
        fn insert_gcd(head: ListNodePtr) -> ListNodePtr {
            head.map(|mut cur| {
                if let Some(next) = cur.next {
                    cur.next = Some(Box::new(ListNode {
                        val: gcd(cur.val, next.val),
                        next: insert_gcd(Some(next)),
                    }));
                }
                cur
            })
        }

        insert_gcd(head)
    }
}

#[cfg(test)]
mod tests {
    use crate::{leet_code::ListNode, leet_code::Solution, linked_list};

    #[test]
    fn test_insert_greatest_common_divisors() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(linked_list!(18, 6, 10, 3)),
            linked_list!(18, 6, 6, 2, 10, 1, 3)
        );
        assert_eq!(Solution::insert_greatest_common_divisors(linked_list!(7)), linked_list!(7));
    }
}
