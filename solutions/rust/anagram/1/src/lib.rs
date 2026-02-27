use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut chars: Vec<char> = lower_word.chars().collect();
    chars.sort();

    let mut res = HashSet::new();
    for candidate in possible_anagrams {
        let lower_candidate = candidate.to_lowercase();
        if lower_candidate != lower_word {
            let mut candidate_chars: Vec<char> = lower_candidate.chars().collect();
            candidate_chars.sort();
            if candidate_chars == chars {
                res.insert(*candidate);
            }
        }
    }
    res
}
