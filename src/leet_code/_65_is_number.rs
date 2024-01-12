use super::Solution;

enum State {
    Start,
    Sign,
    Integer,
    Dot,
    InitialDot,
    Decimal,
    E,
    ESign,
    EInteger,
    Illegal,
}

impl State {
    fn next(&self, next: Input) -> Self {
        match self {
            State::Start => match next {
                Input::Digit => State::Integer,
                Input::Dot => State::InitialDot,
                Input::Sign => State::Sign,
                _ => State::Illegal,
            },
            State::Sign => match next {
                Input::Digit => State::Integer,
                Input::Dot => State::InitialDot,
                _ => State::Illegal,
            },
            State::Integer => match next {
                Input::Digit => State::Integer,
                Input::Dot => State::Dot,
                Input::E => State::E,
                _ => State::Illegal,
            },
            State::Dot => match next {
                Input::Digit => State::Decimal,
                Input::E => State::E,
                _ => State::Illegal,
            },
            State::InitialDot => match next {
                Input::Digit => State::Decimal,
                _ => State::Illegal,
            },
            State::Decimal => match next {
                Input::Digit => State::Decimal,
                Input::E => State::E,
                _ => State::Illegal,
            },
            State::E => match next {
                Input::Digit => State::EInteger,
                Input::Sign => State::ESign,
                _ => State::Illegal,
            },
            State::ESign => match next {
                Input::Digit => State::EInteger,
                _ => State::Illegal,
            },
            State::EInteger => match next {
                Input::Digit => State::EInteger,
                _ => State::Illegal,
            },
            _ => State::Illegal,
        }
    }

    fn accept(&self) -> bool {
        matches!(self, State::Integer | State::Dot | State::Decimal | State::EInteger)
    }
}

enum Input {
    Digit,
    Sign,
    Dot,
    E,
    Other,
}

impl<T> From<T> for Input
where
    T: Into<char>,
{
    fn from(value: T) -> Self {
        match value.into() {
            '0'..='9' => Input::Digit,
            '.' => Input::Dot,
            '+' | '-' => Input::Sign,
            'e' | 'E' => Input::E,
            _ => Input::Other,
        }
    }
}

impl Solution {
    // pub fn _is_number(s: String) -> bool {
    //     s.trim().parse::<f32>().is_ok()
    // }

    pub fn is_number(s: String) -> bool {
        let mut state = State::Start;
        s.chars().for_each(|f| state = state.next(f.into()));
        state.accept()
    }
}

#[cfg(test)]
mod tests {
    use crate::leet_code::Solution;

    #[test]
    fn test_is_number() {
        assert!(Solution::is_number("0".to_string()));
        assert!(Solution::is_number("-123.456e789".to_string()));
        assert!(!Solution::is_number("e".to_string()));
        assert!(!Solution::is_number(".".to_string()));
        assert!(!Solution::is_number("95a54e53".to_string()));
        assert!(!Solution::is_number("99e2.5".to_string()));
    }
}
