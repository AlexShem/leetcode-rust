pub struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let digits = format!("{}", n)
            .chars()
            .map(|d| d.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let sum_of_digits = digits.iter().sum::<i32>();
        let product_of_digits = digits.iter().product::<i32>();

        n % (sum_of_digits + product_of_digits) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_divisibility() {
        assert!(Solution::check_divisibility(99));
        assert!(!Solution::check_divisibility(23));
    }
}
