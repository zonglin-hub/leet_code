use leet_code::leet_code::Solution;

#[test]
fn test_int_to_roman_12_v1() {
    assert_eq!(Solution::int_to_roman_12_v1(3), "III");
    assert_eq!(Solution::int_to_roman_12_v1(4), "IV");
    assert_eq!(Solution::int_to_roman_12_v1(9), "IX");
    assert_eq!(Solution::int_to_roman_12_v1(58), "LVIII");
    assert_eq!(Solution::int_to_roman_12_v1(1994), "MCMXCIV");
}
