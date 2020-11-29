pub fn verse(n: u32) -> String {
    let total_bottles: String;
    let second_total: String;
    let action: String;
    let left_bottles: String;
    match n {
        2 => {
            total_bottles = format!("{} bottles", n);
            second_total = total_bottles.clone();
            action = String::from("Take one down and pass it around");
            left_bottles = String::from("1 bottle");
        },
        1 => {
            total_bottles = String::from("1 bottle");
            second_total = total_bottles.clone();
            action = String::from("Take it down and pass it around");
            left_bottles = String::from("no more bottles");
            
        },
        0 => {
            total_bottles = String::from("No more bottles");
            second_total = String::from("no more bottles");
            action = String::from("Go to the store and buy some more");
            left_bottles = String::from("99 bottles");
        },
        _ => {
            total_bottles = format!("{} bottles", n);
            second_total = total_bottles.clone();
            action = String::from("Take one down and pass it around");
            left_bottles = format!("{} bottles", n-1);
        }
    }

    format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n", total_bottles, second_total, action, left_bottles)
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).map(|x| verse(x)).rev().collect::<Vec<String>>().join(&'\n'.to_string())
    //(start..=end).fold("".to_string(), |mut result, x| {result.push_str(&verse(x)); result})
}
