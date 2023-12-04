//! leet code 算法题

pub mod _1008_bst_from_preorder;
pub mod _105_build_tree;
pub mod _108_sorted_array_to_bst;
pub mod _109_sorted_list_to_bst;
pub mod _10_is_match;
pub mod _1114_print_sequence;
pub mod _112_has_path_sum;
pub mod _114_flatten;
pub mod _118_generate;
pub mod _119_get_row;
pub mod _11_max_area;
pub mod _121_max_profit;
pub mod _1222_queens_attackthe_king;
pub mod _122_max_profit;
pub mod _123_max_profit;
pub mod _12_int_to_roman;
pub mod _1333_filter_restaurants;
pub mod _1367_is_sub_path;
pub mod _136_single_number;
pub mod _13_roman_to_int;
pub mod _147_insertion_sort_list;
pub mod _1491_average;
pub mod _14_longest_common_prefix;
pub mod _1528_restore_string;
pub mod _1545_find_kth_bit;
pub mod _15_three_sum;
pub mod _16_three_sum_closest;
pub mod _17_letter_combinations;
pub mod _18_four_sum;
pub mod _1967_num_of_strings;
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
pub mod _297_c_odec;
pub mod _29_divide;
pub mod _2_add_two_numbers;
pub mod _30_find_substring;
pub mod _31_next_permutation;
pub mod _326_is_power_of_three;
pub mod _328_odd_even_list;
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
pub mod _445_add_two_numbers;
pub mod _44_is_match;
pub mod _45_jump;
pub mod _460_lfu_cache;
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
pub mod _56_merge;
pub mod _57_insert;
pub mod _58_length_of_last_word;
pub mod _59_generate_matrix;
pub mod _5_longest_palindrome;
pub mod _605_can_place_flowers;
pub mod _60_get_permutation;
pub mod _617_merge_trees;
pub mod _61_rotate_right;
pub mod _62_unique_paths;
pub mod _63_unique_paths_with_obstacles;
pub mod _64_min_path_sum;
pub mod _65_is_number;
pub mod _66_plus_one;
pub mod _67_add_binary;
pub mod _680_valid_palindrome;
pub mod _68_full_justify;
pub mod _69_my_sqrt;
pub mod _6_convert;
pub mod _700_search_bst;
pub mod _70_climb_stairs;
pub mod _71_simplify_path;
pub mod _729_book;
pub mod _72_min_distance;
pub mod _73_set_zeroes;
pub mod _74_search_matrix;
pub mod _75_sort_colors;
pub mod _76_min_window;
pub mod _77_combine;
pub mod _78_subsets;
pub mod _79_exist;
pub mod _7_reverse;
pub mod _80_remove_duplicates;
pub mod _81_search;
pub mod _82_delete_duplicates;
pub mod _83_delete_duplicates;
pub mod _84_largest_rectangle_area;
pub mod _856_score_of_parentheses;
pub mod _85_maximal_rectangle;
pub mod _86_partition;
pub mod _87_is_scramble;
pub mod _88_merge;
pub mod _89_gray_code;
pub mod _8_my_atoi;
pub mod _90_subsets_with_dup;
pub mod _91_num_decodings;
pub mod _92_reverse_between;
pub mod _95_generate_trees;
pub mod _96_num_trees;
pub mod _990_equations_possible;
pub mod _9_is_palindrome;
pub mod _lcp_06_min_count;
pub mod _lcp_50_give_gem;
pub mod _offer_06_reverse_print;

use std::{cell::RefCell, rc::Rc};

/// leet code 算法题解
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

/// 链表结构
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodePtr,
}

impl ListNode {
    /// 用于标记一个函数或方法在内部实现时可以被优化掉，从而减少代码量。
    /// 当一个函数或方法被标记为 `#[inline]` 时，Rust编译器会在内部将其替换为等效的机器代码，从而减少调用这个函数的开销。
    /// 需要注意的是，`#[inline]`属性应该仅用于优化性能，而不是用于改变函数的行为或它的可见性。
    /// 在大多数情况下，Rust会自动内联函数调用，因此您不需要显式地使用`#[inline]`属性。
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// 简化 `ListNodePtr`
    #[inline]
    pub fn simplify(val: i32, next: ListNodePtr) -> ListNodePtr {
        Some(Box::new(ListNode { val, next }))
    }
}

#[macro_export]
macro_rules! linked_list {
    () => {
        None
    };
    ($e:expr) => {
        ListNode::simplify($e, None)
    };
    ($e:expr, $($tail:tt)*) => {
        ListNode::simplify($e, linked_list!($($tail)*))
    };
}

pub fn linked_tree(val: i32, left: TreeNodePtr, right: TreeNodePtr) -> TreeNodePtr {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
