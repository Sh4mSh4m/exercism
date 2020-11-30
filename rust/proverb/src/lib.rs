pub fn sentence(word1: &str, word2: &str) -> String {
    format!("For want of a {} the {} was lost.\n", word1.to_string(), word2.to_string())
}

pub fn build_proverb(list: &[&str]) -> String {
    let list_len: usize = list.len();
    if list_len == 0 {
        String::new()
    } else {
        let mut result: String = (0..list_len-1).map(|i| sentence(list[i], list[i+1])).collect::<Vec<String>>().join("");
        result.push_str(&format!("And all for the want of a {}.", list[0]));
        result
    }
}
