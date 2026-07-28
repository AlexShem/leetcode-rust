use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let seen_1 = s1.split_whitespace().fold(HashMap::new(), |mut acc, s| {
            acc.entry(s).and_modify(|v| *v = false).or_insert(true);
            acc
        });
        let seen_2 = s2.split_whitespace().fold(HashMap::new(), |mut acc, s| {
            acc.entry(s).and_modify(|v| *v = false).or_insert(true);
            acc
        });

        let one: Vec<String> = seen_1
            .iter()
            .filter_map(|(&k, v)| {
                if *v && !seen_2.contains_key(k) {
                    Some(k.to_owned())
                } else {
                    None
                }
            })
            .collect();
        let two: Vec<String> = seen_2
            .iter()
            .filter_map(|(&k, v)| {
                if *v && !seen_1.contains_key(k) {
                    Some(k.to_owned())
                } else {
                    None
                }
            })
            .collect();

        [one, two].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let expected = vec!["sweet".to_string(), "sour".to_string()];
        let mut result = Solution::uncommon_from_sentences(s1, s2);
        result.sort();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort();
        assert_eq!(result, expected_sorted);
    }

    #[test]
    fn test_leetcode_example_2() {
        let s1 = "apple apple".to_string();
        let s2 = "banana".to_string();
        let expected = vec!["banana".to_string()];
        let mut result = Solution::uncommon_from_sentences(s1, s2);
        result.sort();
        let mut expected_sorted = expected.clone();
        expected_sorted.sort();
        assert_eq!(result, expected_sorted);
    }
}
