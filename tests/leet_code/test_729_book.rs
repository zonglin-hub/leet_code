use leet_code::leet_code::_729_book::MyCalendar;


#[test]
fn test_book() {
    let mut calendar = MyCalendar::new();
    assert_eq!(calendar.book(10, 20), true);
    assert_eq!(calendar.book(15, 25), false);
    assert_eq!(calendar.book(20, 30), true);
}
