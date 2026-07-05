pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken: Vec<char> = broken_letters.chars().collect();

        text.split_ascii_whitespace()
            .filter(|word| !broken.iter().any(|&c| word.contains(c)))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_examples() {
        assert_eq!(
            Solution::can_be_typed_words("hello world".into(), "ad".into()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".into(), "lt".into()),
            1
        );
        assert_eq!(
            Solution::can_be_typed_words("leet code".into(), "e".into()),
            0
        );
    }
}
