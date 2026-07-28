pub struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|num| {
                let mut n = *num;
                let mut reduced = 0;
                while n > 0 {
                    reduced += n % 10;
                    n /= 10;
                }
                reduced
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let nums = [10, 12, 13, 14];
        assert_eq!(Solution::min_element(nums.into()), 1)
    }

    #[test]
    fn test_leetcode_example_2() {
        let nums = [1, 2, 3, 4];
        assert_eq!(Solution::min_element(nums.into()), 1)
    }

    #[test]
    fn test_leetcode_example_3() {
        let nums = [999, 19, 199];
        assert_eq!(Solution::min_element(nums.into()), 10)
    }
}
