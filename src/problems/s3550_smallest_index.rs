pub struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let search = nums.iter().enumerate().find(|(i, n)| {
            let sum_of_digits = n
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .sum::<u32>();
            *i == sum_of_digits as usize
        });

        match search {
            Some((i, _)) => i as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let input = [1, 3, 2];
        assert_eq!(Solution::smallest_index(input.into()), 2)
    }

    #[test]
    fn test_leetcode_example_2() {
        let input = [1, 10, 11];
        assert_eq!(Solution::smallest_index(input.into()), 1)
    }

    #[test]
    fn test_leetcode_example_3() {
        let input = [1, 2, 3];
        assert_eq!(Solution::smallest_index(input.into()), -1)
    }
}
