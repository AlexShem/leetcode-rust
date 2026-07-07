use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seen = HashSet::new();
        for ch in s.chars() {
            if seen.contains(&ch) {
                return ch;
            }
            seen.insert(ch);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_character() {
        assert_eq!(Solution::repeated_character("abccbaacz".into()), 'c');
        assert_eq!(Solution::repeated_character("abcdd".into()), 'd');
    }
}
