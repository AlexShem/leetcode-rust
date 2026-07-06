pub struct Solution;

impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        n.to_string()
            .chars()
            .filter(|&digit| digit != '0')
            .fold(0, |acc, digit| {
                acc * 10 + digit.to_digit(10).unwrap() as i64
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_zeros() {
        assert_eq!(Solution::remove_zeros(10203), 123);
        assert_eq!(Solution::remove_zeros(10200), 12);
        assert_eq!(Solution::remove_zeros(102000), 12);
        assert_eq!(Solution::remove_zeros(1), 1);
    }
}
