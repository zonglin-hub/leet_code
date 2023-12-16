// //! 餐厅过滤器

// use super::Solution;

// impl Solution {
//     pub fn filter_restaurants(
//         restaurants: Vec<Vec<i32>>,
//         vegan_friendly: i32,
//         max_price: i32,
//         max_distance: i32,
//     ) -> Vec<i32> {
//         restaurants
//             .iter()
//             .filter(|r| r[2] >= vegan_friendly && r[3] <= max_price && r[4] <= max_distance)
//             .collect::<Vec<_>>()
//             .iter()
//             .map(|r| r[0])
//             .collect()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::leet_code::Solution;

//     #[test]
//     fn test_filter_restaurants() {
//         let rest = vec![
//             vec![1, 4, 1, 40, 10],
//             vec![2, 8, 0, 50, 5],
//             vec![3, 8, 1, 30, 4],
//             vec![4, 10, 0, 10, 3],
//             vec![5, 1, 1, 15, 1],
//         ];

//         let mut res = vec![3, 1, 5];
//         res.sort();
//         assert_eq!(Solution::filter_restaurants(rest.clone(), 1, 50, 10), res);

//         let mut res = vec![4, 3, 2, 1, 5];
//         res.sort();
//         assert_eq!(Solution::filter_restaurants(rest.clone(), 0, 50, 10), res);

//         let mut res = vec![4, 5];
//         res.sort();
//         assert_eq!(Solution::filter_restaurants(rest.clone(), 0, 30, 3), res);
//     }
// }
