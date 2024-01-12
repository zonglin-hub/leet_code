use super::Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let week = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];

        let mut days = (year - 1971) * 365 + (year - 1969) / 4;
        days += &month_days[0..month as usize - 1].iter().sum(); // 等同效果
                                                                 // let mut days = 0;
                                                                 // days += 365 * (year - 1971) + (year - 1969) / 4;
                                                                 // for i in 0..(month - 1) {
                                                                 //     days += month_days[i as usize];
                                                                 // }

        if (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) && month >= 3 {
            days += 1;
        }

        days += day;
        week[(days as usize + 3) % 7].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_day_of_the_week() {
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday".to_owned());
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday".to_owned());
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday".to_owned());
        assert_eq!(Solution::day_of_the_week(29, 2, 2000), "Tuesday".to_owned());
    }
}
