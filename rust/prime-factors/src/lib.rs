pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = vec![];
    let mut factor: u64 = 2; 
    while factor < n {
        if n % factor == 0 {
            prime_factors.push(factor);
            n /= factor
        } else {
            factor += 1;
        }
    }
    prime_factors.push(n);
    prime_factors
}

// see recursion solutions and investigate stack overflow for large numbers
// pub fn factors(mut n: u64) -> Vec<u64> {
//    let mut prime_factors = vec![];
//
//     while n > 1 {
//        let i = (2..n+1).find(|x| n % x == 0 ).unwrap();
//        prime_factors.push(i);
//        n /= i;
//    }
//    prime_factors
//}
