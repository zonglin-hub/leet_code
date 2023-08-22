# Leet Code

使用 Rust 解决 Leetcode Code 

先写 Docs，再写测试，以测试驱动完成 Code

## Table of Contents



| id            | Leet Code                                                         | 标签                                        |
| ------------- | ----------------------------------------------------------------- | ------------------------------------------- |
| 1             | [两数之和](./src/_1_two_sum.rs)                                   | `数组` `哈希表`                             |
| 2             | [两数相加](./src/_2_add_two_numbers.rs)                           | `递归` `链表` `数学`                        |
| 3             | [无重复字符的最长子串](./src/_3_length_of_longest_substring.rs)   | `哈希表` `字符串` `滑动窗口`                |
| 4             | [寻找两个正序数组的中位数](./src/_4_find_median_sorted_arrays.rs) | `数组` `二分查找` `分治`                    |
| 5             | [最长回文子串](./src/_5_longest_palindrome.rs)                    | `字符串` `动态规划`                         |
| 6             | [N 字形变换](./src/_6_convert.rs)                                 | `字符串`                                    |
| 7             | [整数反转](./src/_7_reverse.rs)                                   | `数学`                                      |
| 8             | [字符串转换整数 (atoi)](./src/_8_my_atoi.rs)                      | `字符串`                                    |
| 9             | [回文数](./src/_9_is_palindrome.rs)                               | `数学`                                      |
| 10            | [正则表达式匹配](./src/_10_is_match.rs)                           | `递归` `字符串` `动态规划`                  |
| 11            | [盛最多水的容器](./src/_11_max_area.rs)                           | `贪心` `数组` `双指针`                      |
| 12            | [整数转罗马数字](./src/_12_int_to_roman.rs)                       | `哈希表` `数学` `字符串`                    |
| 13            | [罗马数字转整数](./src/_13_roman_to_int.rs)                       | `哈希表` `数学` `字符串`                    |
| 14            | [最长公共前缀](./src/_14_longest_common_prefix.rs)                | `字典树` `字符串`                           |
| 15            | [三数之和](./src/_15_three_sum.rs)                                | `数组` `双指针` `排序`                      |
| 16            | [最接近的三数之和](./src/_16_three_sum_closest.rs)                | `数组` `双指针` `排序`                      |
| 17            | [电话号码的字母组合](./src/_17_letter_combinations.rs)            | `哈希表` `字符串` `回溯`                    |
| 18            | [四数之和](./src/_18_four_sum.rs)                                 | `数组` `双指针` `排序`                      |
| 19            | [删除链表的倒数第 N 个结点](./src/_19_remove_nth_from_end.rs)     | `链表` `双指针`                             |
| 20            | [有效的括号](./src/_20_is_valid.rs)                               | `栈` `字符串`                               |
| 21            | [合并两个有序链表](./src/_21_merge_two_lists.rs)                  | `递归` `链表`                               |
| 22            | [括号生成](./src/_22_generate_parenthesis.rs)                     | `字符串` `动态规划` `回溯`                  |
| 23            | [合并 K 个升序链表](./src/_23_merge_k_lists.rs)                   | `链表` `分治` `堆（优先队列）` `归并排序`   |
| 24            | [两两交换链表中的节点](./src/_24_swap_pairs.rs)                   | `递归` `链表`                               |
| 25            | [K 个一组翻转链表](./src/_25_reverse_k_group.rs)                  | `递归` `链表`                               |
| 26            | [删除有序数组中的重复项](./src/_26_remove_duplicates.rs)          | `数组` `双指针`                             |
| 27            | [移除元素](./src/_27_remove_element.rs)                           | `数组` `双指针`                             |
| 49            | [字母异位词分组](./src/_49_group_anagrams.rs)                     | `数组` `哈希表` `字符串` `排序`             |
| 70            | [爬楼梯](./src/_70_climb_stairs.rs)                               | `记忆化搜索` `数学` `动态规划`              |
| 112           | [路径总和](./src/_112_has_path_sum.rs)                            | `树` `深度优先搜索` `广度优先搜索` `二叉树` |
| 118           | [杨辉三角](./src/_118_generate.rs)                                | `数组` `动态规划`                           |
| 119           | [杨辉三角 II](./src/_119_get_row.rs)                              | `数组` `动态规划`                           |
| 121           | [买卖股票的最佳时机](./src/_112_has_path_sum.rs)                  | `数组` `动态规划`                           |
| 123           | [买卖股票的最佳时机 III](./src/_123_max_profit.rs)                | `数组` `动态规划`                           |
| 136           | [只出现一次的数字](./src/_136_single_number.rs)                   | `位运算` `数组`                             |
| 203           | [移除链表元素](./src/_203_remove_elements.rs)                     | `递归` `链表`                               |
| 206           | [反转链表](./src/_206_reverse_list.rs)                            | `递归` `链表`                               |
| 231           | [2 的幂](./src/_231_is_power_of_two.rs)                           | `位运算` `递归` `数学`                      |
| 234           | [回文链表](./src/_234_is_palindrome.rs)                           | `栈` `递归` `链表` `双指针`                 |
| 326           | [3 的幂](./src/_326_is_power_of_three.rs)                         | `递归` `数学`                               |
| 338           | [比特位计数](./src/_338_count_bits.rs)                            | `位运算` `动态规划`                         |
| 342           | [4的幂](./src/_342_is_power_of_four.rs)                           | `位运算` `递归` `数学`                      |
| 392           | [判断子序列](./src/_392_is_subsequence.rs)                        | `双指针` `字符串` `动态规划`                |
| 415           | [字符串相加](./src/_415_add_strings.rs)                           | `数学` `字符串` `模拟`                      |
| 509           | [斐波那契数](./src/_509_fib.rs)                                   | `递归` `记忆化搜索` `数学` `动态规划`       |
| 617           | [合并二叉树](./src/_617_merge_trees.rs)                           | `树` `深度优先搜索` `广度优先搜索` `二叉树` |
| 729           | [我的日程安排表 I](./src/_729_book.rs)                            | `设计` `线段树` `二分查找` `有序集合`       |
| 1491          | [去掉最低工资和最高工资后的工资平均值](./src/_1491_average.rs)    | `数组` `排序`                               |
| 1545          | [找出第 N 个二进制字符串中的第 K 位](./src/_1545_find_kth_bit.rs) | `递归` `字符串`                             |
| 2224          | [转化时间需要的最少操作数](./src/_2224_convert_time.rs)           | `贪心` `字符串`                             |
| 2472          | [不重叠回文子字符串的最大数目](./src/_2472_max_palindromes.rs)    | `动态规划` `字符串`                         |
| 剑指 Offer 06 | [从尾到头打印链表](./src/offer_06_reverse_print.rs)               | `栈` `递归` `链表` `双指针`                 |
| 剑指 Offer 24 | [反转链表](./src/offer_24_reverse_list.rs)                        | `递归` `链表`                               |
| 剑指 Offer 25 | [合并两个排序的链表](./src/offer_25_merge_two_lists.rs)           | `递归` `链表`                               |
| 剑指 Offer 42 | [连续子数组的最大和](./src/offer_42_max_sub_array.rs)             | `数组` `分治` `动态规划`                    |





## 参考

- [Rust GYM](https://rustgym.com/)
- [力扣（LeetCode）官网 - 全球极客挚爱的技术成长平台](https://leetcode.cn)