use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut frequencies: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            frequencies
                .entry(num)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }
        let max_freq = frequencies.values().max().copied().unwrap_or(0);
        frequencies.values().filter(|&&v| v == max_freq).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_frequency_elements() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
