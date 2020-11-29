trait Prime {
    fn is_prime(self) -> bool;
}

impl Prime for u32 {
    fn is_prime(self) -> bool {
            !(2..self).any(|x| self % x == 0)
        }
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| x.is_prime()).nth(n as usize).unwrap()
}
