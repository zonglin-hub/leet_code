//! 我的日程安排表 I

#![allow(unused)]

use std::collections::BTreeSet;

struct MyCalendar {
    bts: BTreeSet<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self {
            bts: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some(v) = self.bts.range((start, 0)..).next() {
            if v.1 < end {
                return false;
            }
        }
        self.bts.insert((end - 1, start));
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::_729_book::MyCalendar;

    #[test]
    fn test_book() {
        /*
            输入：
            ["MyCalendar", "book", "book", "book"]
            [[], [10, 20], [15, 25], [20, 30]]
            输出：
            [null, true, false, true]

            解释：
            MyCalendar myCalendar = new MyCalendar();
            myCalendar.book(10, 20); // return True
            myCalendar.book(15, 25); // return False ，这个日程安排不能添加到日历中，因为时间 15 已经被另一个日程安排预订了。
            myCalendar.book(20, 30); // return True ，这个日程安排可以添加到日历中，因为第一个日程安排预订的每个时间都小于 20 ，且不包含时间 20 。
        */
        let mut calendar = MyCalendar::new();
        assert_eq!(calendar.book(10, 20), true);
        assert_eq!(calendar.book(15, 25), false);
        assert_eq!(calendar.book(20, 30), true);
    }
}
