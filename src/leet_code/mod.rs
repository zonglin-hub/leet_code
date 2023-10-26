//! leet code 算法题

pub mod _1008_bst_from_preorder;
pub mod _105_build_tree;
pub mod _108_sorted_array_to_bst;
pub mod _10_is_match;
pub mod _1114_print_sequence;
pub mod _112_has_path_sum;
pub mod _118_generate;
pub mod _119_get_row;
pub mod _11_max_area;
pub mod _121_max_profit;
pub mod _1222_queens_attackthe_king;
pub mod _123_max_profit;
pub mod _12_int_to_roman;
pub mod _1333_filter_restaurants;
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
pub mod _198_rob;
pub mod _19_remove_nth_from_end;
pub mod _1_two_sum;
pub mod _203_remove_elements;
pub mod _206_reverse_list;
pub mod _2086_minimum_buckets;
pub mod _20_is_valid;
pub mod _2136_earliest_full_bloom;
pub mod _213_rob;
pub mod _21_merge_two_lists;
pub mod _2224_convert_time;
pub mod _2231_largest_integer;
pub mod _22_generate_parenthesis;
pub mod _231_is_power_of_two;
pub mod _234_is_palindrome;
pub mod _2395_find_subarrays;
pub mod _23_merge_k_lists;
pub mod _2472_max_palindromes;
pub mod _24_swap_pairs;
pub mod _2525_categorize_box;
pub mod _2560_min_capability;
pub mod _2562_find_the_array_conc_val;
pub mod _2578_split_num;
pub mod _2582_pass_the_pillow;
pub mod _2591_dist_money;
pub mod _25_reverse_k_group;
pub mod _2603_collect_the_coins;
pub mod _2672_color_the_array;
pub mod _26_remove_duplicates;
pub mod _2731_sum_distance;
pub mod _27_remove_element;
pub mod _2831_longest_equal_subarray;
pub mod _2840_check_strings;
pub mod _28_str_str;
pub mod _29_divide;
pub mod _2_add_two_numbers;
pub mod _30_find_substring;
pub mod _31_next_permutation;
pub mod _326_is_power_of_three;
pub mod _32_longest_valid_parentheses;
pub mod _337_rob;
pub mod _338_count_bits;
pub mod _33_search;
pub mod _342_is_power_of_four;
pub mod _34_search_range;
pub mod _35_search_insert;
pub mod _36_is_valid_sudoku;
pub mod _37_solve_sudoku;
pub mod _383_can_construct;
pub mod _38_count_and_say;
pub mod _392_is_subsequence;
pub mod _39_combination_sum;
pub mod _3_length_of_longest_substring;
pub mod _40_combination_sum2;
pub mod _415_add_strings;
pub mod _41_first_missing_positive;
pub mod _42_trap;
pub mod _43_multiply;
pub mod _44_is_match;
pub mod _45_jump;
pub mod _46_permute;
pub mod _47_permute_unique;
pub mod _48_rotate;
pub mod _49_group_anagrams;
pub mod _4_find_median_sorted_arrays;
pub mod _509_fib;
pub mod _50_my_pow;
pub mod _51_solve_n_queens;
pub mod _526_count_arrangement;
pub mod _52_total_n_queens;
pub mod _53_max_sub_array;
pub mod _54_spiral_order;
pub mod _55_can_jump;
pub mod _5_longest_palindrome;
pub mod _605_can_place_flowers;
pub mod _617_merge_trees;
pub mod _680_valid_palindrome;
pub mod _6_convert;
pub mod _70_climb_stairs;
pub mod _729_book;
pub mod _7_reverse;
pub mod _856_score_of_parentheses;
pub mod _8_my_atoi;
pub mod _9_is_palindrome;
pub mod _lcp_06_min_count;
pub mod _lcp_50_give_gem;
pub mod _offer_06_reverse_print;

use std::{cell::RefCell, rc::Rc};

/// leet code 算法解题
pub struct Solution;

type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;
type ListNodePtr = Option<Box<ListNode>>;

/// 定义二叉树节点
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeNodePtr,
    pub right: TreeNodePtr,
}

impl TreeNode {
    // 创建新节点
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// `vector` 转 `Option<Rc<RefCell<TreeNode>>>`
pub fn array_to_tree(root: Vec<Option<i32>>) -> TreeNodePtr {
    if root.is_empty() {
        return None;
    }
    let root_node = Rc::new(RefCell::new(TreeNode::new(root[0].expect(""))));
    let mut queue = vec![Rc::clone(&root_node)];
    let mut i = 1;
    while i < root.len() {
        let node = queue.remove(0);
        if let Some(val) = root[i] {
            let left = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = Some(Rc::clone(&left));
            queue.push(left);
        }
        i += 1;
        if i < root.len() && root[i].is_some() {
            let val = root[i].expect("");
            let right = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().right = Some(Rc::clone(&right));
            queue.push(right);
        }
        i += 1;
    }
    Some(root_node)
}

/// 链表结构
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodePtr,
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

/// 简化 `tree` 套娃 -> `Option<Rc<RefCell<TreeNode>>>`
pub fn create_tree_node(val: i32, left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

/// 简化链表套娃 -> `Option<Box<ListNode>>`
pub fn create_list_node(nums: i32) -> ListNodePtr {
    Some(Box::new(ListNode {
        val: nums,
        next: None,
    }))
}

/// `vector` 转 `Option<Box<ListNode>>`
pub fn create_list(nums: Vec<i32>) -> ListNodePtr {
    if nums.is_empty() {
        return None;
    }
    let mut head = create_list_node(nums[0]);
    let mut p = head.as_mut();
    for num in nums.iter().skip(1) {
        let node = create_list_node(*num);
        p.as_mut().expect("").next = node;
        p = p.expect("").next.as_mut();
    }
    head
}

/// 用于测试数组乱序情况
///
/// assertion failed: `(left == right)`
///  left: `[[0, 1], [3, 3], [1, 0]]`,
///  right: `[[0, 1], [1, 0], [3, 3]]`
pub fn expected_sort(queens: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut expected = queens.iter().map(|e| e.to_vec()).collect::<Vec<Vec<i32>>>();
    expected.sort();
    expected
}

/// `vec![[0, 1]]` 转 `vec![vec![0, 1]]`
pub fn expected_sort_vec(queens: Vec<[i32; 2]>) -> Vec<Vec<i32>> {
    queens.iter().map(|&x| x.to_vec()).collect()
}

pub fn expected_sort_vec_char(queens: Vec<[char; 9]>) -> Vec<Vec<char>> {
    queens.iter().map(|x| x.to_vec()).collect()
}

/// `ListNode` 转 `vector`
pub fn to_vec(head: ListNodePtr) -> Vec<i32> {
    let mut res = vec![];
    let mut p = head;
    while let Some(node) = p {
        res.push(node.val);
        p = node.next;
    }
    res
}

/// `&str` 转 `vector`
pub fn to_int_vec(s: &str) -> Vec<i32> {
    s.bytes().map(|x| (x - b'0') as i32).rev().collect()
}
