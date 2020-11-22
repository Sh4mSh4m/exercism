trait Divisible {
    fn is_divisible_by(&self, num: u32) -> bool;
}

impl Divisible for u32{
    fn is_divisible_by(&self, num: u32) -> bool {
        *&self % num == 0
    }
}

pub fn raindrops(n: u32) -> String {
    let mut rain = String::from("");
    let pling = String::from("Pling");
    let plang = String::from("Plang");
    let plong = String::from("Plong");

    if n.is_divisible_by(3) {
        rain += &pling;
    }
    if n.is_divisible_by(5) {
        rain += &plang;
    }
    if n.is_divisible_by(7) {
        rain += &plong;
    }
    if !n.is_divisible_by(3) && 
        !n.is_divisible_by(5) &&
         !n.is_divisible_by(7) {
             rain += &n.to_string();
         }
    rain
}

// in yun-cloud's solution. everything is functional
// Question: what is && or & in closures ? 
// Answer: iter on reference, therefore two &&
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
//
// pub fn raindrops(num: i64) -> String {
//    let outputs: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
//        .into_iter()
//        .filter(|&&(n, _)| num % n == 0)    // whether n is num's factor
//        .map(|&(_, s)| s)                   // we only need the str
//        .collect();
//    if outputs.len() != 0 {
//        outputs
//    } else {
//        num.to_string()
//    }
//}
