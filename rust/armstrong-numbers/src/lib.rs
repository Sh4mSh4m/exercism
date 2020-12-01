use std::convert::TryInto;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: String = num.to_string();
    let number: u32 = digits.len().try_into().unwrap();
    num == digits.chars().map(|x| x.to_digit(10).unwrap().pow(number)).sum()
}
