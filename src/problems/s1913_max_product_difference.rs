pub struct Solution;

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let (a, b, c, d) = (nums[0], nums[1], nums[n - 1], nums[n - 2]);
        return c * d - a * b;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let nums = [5, 6, 2, 7, 4];
        assert_eq!(Solution::max_product_difference(nums.into()), 34)
    }

    #[test]
    fn test_leetcode_example_2() {
        let nums = [4, 2, 5, 9, 7, 4, 8];
        assert_eq!(Solution::max_product_difference(nums.into()), 64)
    }
}
