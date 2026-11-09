use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        return Solution::count_chars(&s) == Solution::count_chars(&t);
    }

    fn count_chars(s: &str) -> HashMap<char, i32> {
        let mut freq: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            freq.entry(c).and_modify(|key| *key += 1).or_insert(1);
        }
        freq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(Solution::is_anagram(s, t))
    }

    #[test]
    fn test_leetcode_example_2() {
        let s = String::from("rat");
        let t = String::from("car");
        assert!(!Solution::is_anagram(s, t))
    }
}
