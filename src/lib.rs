//! 使用 Rust 解决 Leetcode 相关问题。
//!
//! 先写测试模块，以测试驱动完成 Rust Code。
//!

pub mod _105_build_tree;
pub mod _108_sorted_array_to_bst;
pub mod _10_is_match;
pub mod _1114_print_sequence;
pub mod _112_has_path_sum;
pub mod _118_generate;
pub mod _119_get_row;
pub mod _11_max_area;
pub mod _121_max_profit;
pub mod _123_max_profit;
pub mod _12_int_to_roman;
pub mod _136_single_number;
pub mod _13_roman_to_int;
pub mod _1491_average;
pub mod _14_longest_common_prefix;
pub mod _1528_restore_string;
pub mod _1545_find_kth_bit;
pub mod _15_three_sum;
pub mod _16_three_sum_closest;
pub mod _17_letter_combinations;
pub mod _18_four_sum;
pub mod _19_remove_nth_from_end;
pub mod _1_two_sum;
pub mod _203_remove_elements;
pub mod _206_reverse_list;
pub mod _2086_minimum_buckets;
pub mod _20_is_valid;
pub mod _21_merge_two_lists;
pub mod _2224_convert_time;
pub mod _2231_largest_integer;
pub mod _22_generate_parenthesis;
pub mod _231_is_power_of_two;
pub mod _234_is_palindrome;
pub mod _23_merge_k_lists;
pub mod _2472_max_palindromes;
pub mod _24_swap_pairs;
pub mod _25_reverse_k_group;
pub mod _26_remove_duplicates;
pub mod _27_remove_element;
pub mod _2831_longest_equal_subarray;
pub mod _28_str_str;
pub mod _29_divide;
pub mod _2_add_two_numbers;
pub mod _30_find_substring;
pub mod _31_next_permutation;
pub mod _326_is_power_of_three;
pub mod _32_longest_valid_parentheses;
pub mod _338_count_bits;
pub mod _342_is_power_of_four;
pub mod _35_search_insert;
pub mod _392_is_subsequence;
pub mod _3_length_of_longest_substring;
pub mod _415_add_strings;
pub mod _49_group_anagrams;
pub mod _4_find_median_sorted_arrays;
pub mod _509_fib;
pub mod _50_my_pow;
pub mod _53_max_sub_array;
pub mod _5_longest_palindrome;
pub mod _617_merge_trees;
pub mod _6_convert;
pub mod _70_climb_stairs;
pub mod _729_book;
pub mod _7_reverse;
pub mod _856_score_of_parentheses;
pub mod _8_my_atoi;
pub mod _9_is_palindrome;
pub mod offer_06_reverse_print;

use std::{cell::RefCell, rc::Rc};

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // 用于标记一个函数或方法在内部实现时可以被优化掉，从而减少代码量。
    // 当一个函数或方法被标记为#[inline]时，Rust编译器会在内部将其替换为等效的机器代码，从而减少调用这个函数的开销。
    // 需要注意的是，#[inline]属性应该仅用于优化性能，而不是用于改变函数的行为或它的可见性。
    // 在大多数情况下，Rust会自动内联函数调用，因此您不需要显式地使用#[inline]属性。
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_tree_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.is_empty() {
        return None;
    }

    // 创建一个头节点，并将其赋值给head
    let mut head = Some(Box::new(ListNode::new(nums[0])));
    // 将head赋值给p
    let mut p = head.as_mut();
    // 遍历nums数组，将每一个元素赋值给ListNode
    for num in nums.iter().skip(1) {
        let node = Some(Box::new(ListNode::new(*num)));
        // 将ListNode赋值给p的下一个节点
        p.as_mut().expect("").next = node;
        // 将p的下一个节点赋值给p
        p = p.expect("").next.as_mut();
    }
    // 返回head
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree_node() {
        let node = create_tree_node(1, None, None);
        assert_eq!(node.expect("").borrow().val, 1);
    }
}
