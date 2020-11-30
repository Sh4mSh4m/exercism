pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| factors.iter().any(|y| {if *y==0 {false} else {x % y == 0}} )).sum::<u32>()
}
