use super::Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        let mut ans = -1;
        let mut max_profit = 0;
        let mut total_profit = 0;
        let mut operations = 0;
        let mut customers_count = 0;

        for i in customers {
            operations += 1;
            customers_count += i;
            let cur_customers = customers_count.min(4);
            customers_count -= cur_customers;
            total_profit += boarding_cost * cur_customers - running_cost;
            if total_profit > max_profit {
                max_profit = total_profit;
                ans = operations;
            }
        }

        if customers_count == 0 {
            return ans;
        }

        let profit_each_time = boarding_cost * 4 - running_cost;
        if profit_each_time <= 0 {
            return ans;
        }

        if customers_count > 0 {
            let full_times = customers_count / 4;
            total_profit += profit_each_time * full_times;
            operations += full_times;
            if total_profit > max_profit {
                max_profit = total_profit;
                ans = operations;
            }
            let remaining_customers = customers_count % 4;
            let remaining_profit = boarding_cost * remaining_customers - running_cost;
            total_profit += remaining_profit;
            if total_profit > max_profit {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_min_operations_max_profit() {
        assert_eq!(Solution::min_operations_max_profit(vec![8, 3], 5, 6), 3);
        assert_eq!(Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4), 7);
        assert_eq!(Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92), -1);
    }
}
