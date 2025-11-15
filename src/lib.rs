use std::collections::HashSet;

pub fn analyze_text(text: &str) -> (usize, usize, String) {
    let mut count: usize = 0;
    let mut uniq_words: HashSet<String> = HashSet::new();
    let mut longest: &str = "";

    for word in text.split_whitespace() {
        count = count + 1;

        if word.len() > longest.len() {
            longest = word;
        }

        uniq_words.insert(word.to_lowercase());
    }

    (count, uniq_words.len(), String::from(longest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "hello world hello again";
        assert_eq!(analyze_text(text), (4, 3, "hello".to_string()));
    }
}
