pub struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|digit| digit % 3 == 0 || digit % 5 == 0 || digit % 7 == 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_multiples() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
        assert_eq!(Solution::sum_of_multiples(10), 40);
        assert_eq!(Solution::sum_of_multiples(9), 30);
    }
}
