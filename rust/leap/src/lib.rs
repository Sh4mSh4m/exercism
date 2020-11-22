trait Divisible {
    fn is_divisible_by(&self, number:u64) -> bool;
}

impl Divisible for u64 {
    fn is_divisible_by(&self, number: u64) -> bool {
        *&self % number == 0
    }
}

// question why did it work without the borrowing & of self ?
// Wouldn't it consume self ?
//


pub fn is_leap_year(year:u64) -> bool {
    year.is_divisible_by(4) && !year.is_divisible_by(100) 
        || year.is_divisible_by(400)
}
pub fn old_is_leap_year(year: u64) -> bool {
    // unimplemented!("true if {} is a leap year", year)
    if (year % 4 == 0 && year % 100 != 0) || (year % 100 == 0 && year % 400 == 0) {
         true
    } else {
         false
    }
}
