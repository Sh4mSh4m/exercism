use std::collections::{HashSet};

pub fn sorted_lowercase_anagram(word: &str) -> Vec<char> {
    let mut chars_vec = word.to_lowercase().chars().collect::<Vec<char>>();
    chars_vec.sort();
    chars_vec
}


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter()
        .filter(|&p_word| p_word.to_lowercase() != word.to_lowercase())
        .filter(|&p_word| sorted_lowercase_anagram(p_word) == sorted_lowercase_anagram(word))
        .map(|x| *x)
        .collect::<HashSet<&str>>()
}


// Former version: complicated, does not pass case sensitive test...
//pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//    let word_len = word.len();
//    let word_hashmap = word.chars()
//        .fold(HashMap::new(), |mut map, char| {map.insert(char, word.chars().filter(|c| c == &char).count()); map});
//    possible_anagrams.iter()
//        .filter(|p_word| p_word.len() == word_len && *p_word.to_lowercase() != word.to_lowercase()[..])
//        .filter(|p_word| word_hashmap.iter().all(|(char, len)| p_word.chars().filter(|c| c.eq_ignore_ascii_case(&char)).count() == *len))
//        .map(|x| *x)
//        .collect::<HashSet<&str>>()
//}
