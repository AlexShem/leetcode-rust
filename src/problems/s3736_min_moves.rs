pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max_value = nums.iter().max().unwrap();
        let target = max_value * (nums.len() as i32);
        target - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leetcode_exmaple_1() {
        let nums = [2, 1, 3];
        assert_eq!(Solution::min_moves(nums.into()), 3);
    }

    #[test]
    fn test_leetcode_exmaple_2() {
        let nums = [4, 4, 5];
        assert_eq!(Solution::min_moves(nums.into()), 2);
    }
}
