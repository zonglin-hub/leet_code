//! 餐厅过滤器

use super::Solution;

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let res = restaurants
            .iter()
            .filter(|r| r[2] >= vegan_friendly && r[3] <= max_price && r[4] <= max_distance)
            .collect::<Vec<_>>();

        res.iter().map(|r| r[0]).collect()
    }
}
