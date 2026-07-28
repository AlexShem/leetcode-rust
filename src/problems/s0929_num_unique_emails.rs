use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .fold(HashSet::new(), |mut acc, email| {
                let (local, domain) = email.split_once('@').unwrap();
                let local_name: String = match local.split_once('+') {
                    Some((s, _)) => s.to_string(),
                    None => local.to_string(),
                }
                .chars()
                .filter(|c| *c != '.')
                .collect();
                let address = format!("{}@{}", local_name, domain);
                acc.insert(address);
                acc
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let emails: Vec<String> = vec![
            "test.email+alex@leetcode.com".to_owned(),
            "test.e.mail+bob.cathy@leetcode.com".to_owned(),
            "testemail+david@lee.tcode.com".to_owned(),
        ];
        assert_eq!(Solution::num_unique_emails(emails), 2)
    }

    #[test]
    fn test_leetcode_example_2() {
        let emails: Vec<String> = vec![
            "a@leetcode.com".to_owned(),
            "b@leetcode.com".to_owned(),
            "c@leetcode.com".to_owned(),
        ];
        assert_eq!(Solution::num_unique_emails(emails), 3)
    }

    #[test]
    fn test_leetcode_example_3() {
        let emails: Vec<String> = vec![
            "test.email+alex@leetcode.com".to_owned(),
            "test.email@leetcode.com".to_owned(),
        ];
        assert_eq!(Solution::num_unique_emails(emails), 1)
    }
}
