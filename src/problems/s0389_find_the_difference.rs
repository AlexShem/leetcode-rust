use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let chars_s: HashMap<char, i32> = s.chars().fold(HashMap::new(), |mut acc, ch| {
            acc.entry(ch).and_modify(|val| *val += 1).or_insert(1);
            acc
        });
        let chars_t: HashMap<char, i32> = t.chars().fold(HashMap::new(), |mut acc, ch| {
            acc.entry(ch).and_modify(|val| *val += 1).or_insert(1);
            acc
        });

        for (k, v) in chars_t {
            if !chars_s.contains_key(&k) || *chars_s.get(&k).unwrap() != v {
                return k;
            }
        }
        panic!("No difference found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_difference() {
        assert_eq!(
            Solution::find_the_difference("abcd".into(), "abcde".into()),
            'e'
        );
        assert_eq!(Solution::find_the_difference("".into(), "y".into()), 'y');
    }
}
