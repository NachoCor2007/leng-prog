use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(phrase: &str) -> HashMap<String, u32> {
  let words = phrase.split_whitespace();
  let mut map = HashMap::new();

  for word in words {
    let un_capitalized_word = word.to_lowercase();

    let word_entry = map.entry(un_capitalized_word).or_insert(0);
    *word_entry += 1;
  }

  map
}
