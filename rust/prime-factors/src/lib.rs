pub fn factors(n: u64) -> Vec<u64> {
    //unimplemented!("This should calculate the prime factors of {}", n)
    let mut prime_factors: Vec<u64> = vec![n];
    let mut quotient: u64 = prime_factors.pop().unwrap();
    let mut factor: u64 = 2; 
    while factor < quotient {
        if quotient % factor == 0 {
            prime_factors.push(factor);
            prime_factors.push(quotient / factor);
            quotient = prime_factors.pop().unwrap(); 
        } else {
            factor += 1;
        }
    }
    prime_factors.push(quotient);
    prime_factors
}


// see recursion solutions and investigate stack overflow for large numbers
