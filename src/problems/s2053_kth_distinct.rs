use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let seen = arr.iter().fold(HashMap::new(), |mut acc, s| {
            acc.entry(s).and_modify(|v| *v = false).or_insert(true);
            acc
        });

        arr.iter()
            .filter(|s| *seen.get(s).unwrap())
            .nth((k - 1) as usize)
            .unwrap_or(&String::new())
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let arr = vec![
            "d".into(),
            "b".into(),
            "c".into(),
            "b".into(),
            "c".into(),
            "a".into(),
        ];
        let k = 2;
        assert_eq!(Solution::kth_distinct(arr, k), "a".to_string());
    }

    #[test]
    fn test_leetcode_example_2() {
        let arr = vec!["aaa".into(), "aa".into(), "a".into()];
        let k = 1;
        assert_eq!(Solution::kth_distinct(arr, k), "aaa".to_string());
    }

    #[test]
    fn test_leetcode_example_3() {
        let arr = vec!["a".into(), "b".into(), "a".into()];
        let k = 3;
        assert_eq!(Solution::kth_distinct(arr, k), "".to_string());
    }
}
