pub fn square(s: u32) -> u64 {
    //unimplemented!("grains of rice on square {}", s);
    if 1 <= s && s <= 64 {
        2u64.pow(s-1)
    } else {
        panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum::<u64>()
}

// see solution with binary operator leftshift on integers
//pub fn square(s: u32) -> u64 {
//	if s < 1 || s > 64 {
//		panic!("Square must be between 1 and 64")
//	} else {
//		1u64 << (s - 1)
//	}
//}
//
//pub fn total() -> u64 {
//	u64::max_value()
//}
