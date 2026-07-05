pub struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (2..=num)
            .filter(|n| {
                let sum_of_digits = n
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap_or_default())
                    .sum::<u32>();
                sum_of_digits % 2 == 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        assert_eq!(Solution::count_even(4), 2);
    }

    #[test]
    fn test_leetcode_example_2() {
        assert_eq!(Solution::count_even(30), 14);
    }
}
