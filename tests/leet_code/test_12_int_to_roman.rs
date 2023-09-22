use leet_code::leet_code::Solution;

#[test]
fn test_int_to_roman_12_v1() {
    assert_eq!(Solution::int_to_roman_12_v1(3), "III");
    assert_eq!(Solution::int_to_roman_12_v1(4), "IV");
    assert_eq!(Solution::int_to_roman_12_v1(9), "IX");
    assert_eq!(Solution::int_to_roman_12_v1(58), "LVIII");
    assert_eq!(Solution::int_to_roman_12_v1(1994), "MCMXCIV");
}

#[test]
fn test_roman_to_int_13_v1() {
    assert_eq!(Solution::roman_to_int_13_v1("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int_13_v1("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int_13_v1("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int_13_v1("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int_13_v1("MCMXCIV".to_string()), 1994);
}
