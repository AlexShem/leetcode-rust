pub struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let non_zero: Vec<i64> = n
            .to_string()
            .chars()
            .filter_map(|ch| {
                ch.to_digit(10)
                    .and_then(|digit| if digit == 0 { None } else { Some(digit as i64) })
            })
            .collect();
        non_zero.iter().sum::<i64>() * non_zero.iter().fold(0, |acc, digit| acc * 10 + digit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_and_multiply() {
        assert_eq!(Solution::sum_and_multiply(10203004), 12340);
        assert_eq!(Solution::sum_and_multiply(1000), 1);
    }
}
