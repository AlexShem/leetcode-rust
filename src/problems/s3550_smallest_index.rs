pub struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .find(|&(i, n)| {
                std::iter::successors(Some(n), |n| Some(n / 10))
                    .map_while(|n| (n > 0).then_some(n % 10))
                    .sum::<i32>()
                    == i as i32
            })
            .map_or(-1, |(i, _)| i as i32)
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
