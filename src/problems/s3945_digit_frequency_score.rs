pub struct Solution;

impl Solution {
    pub fn digit_frequency_score(n: i32) -> i32 {
        std::iter::successors(Some(n), |k| Some(k / 10))
            .take_while(|val| val.is_positive())
            .fold(0, |acc, n| acc + n % 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_frequency_score() {
        assert_eq!(Solution::digit_frequency_score(122), 5);
        assert_eq!(Solution::digit_frequency_score(101), 2);
    }
}
