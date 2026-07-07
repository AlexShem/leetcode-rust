pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, num| {
            let steps = if num.rem_euclid(3) == 0 { 0 } else { 1 };
            acc + steps
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
    }
}
