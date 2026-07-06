pub struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_str = format!("{}", num);
        let k = k as usize;

        (0..=num_str.len() - k)
            .map(|i| &num_str[i..i + k])
            .filter_map(|substr| substr.parse::<i32>().ok())
            .filter(|&divisor| divisor != 0 && num % divisor == 0)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
    }

    #[test]
    fn test_leetcode_example_2() {
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
    }
}
