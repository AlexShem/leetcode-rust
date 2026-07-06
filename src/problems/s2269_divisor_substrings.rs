pub struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_str = format!("{}", num);
        let k = k as usize;
        let mut count = 0;
        for i in 0..=num_str.len() - (k as usize) {
            let substr = &num_str.as_str()[i..i + k];
            let divisor = substr.parse::<i32>().unwrap_or(0);
            if divisor != 0 && num % divisor == 0 {
                count += 1;
            }
        }
        count
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
