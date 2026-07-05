pub struct Solution;

impl Solution {
    pub fn sort_by_reflection(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|&n| ((n as u32).reverse_bits() >> n.leading_zeros(), n));
        nums
    }

    #[allow(unused)]
    fn get_reflection(n: i32) -> i32 {
        let binary = format!("{:b}", n);
        let reversed = binary.chars().rev().collect::<String>();
        i32::from_str_radix(&reversed, 2).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_reflection() {
        assert_eq!(Solution::get_reflection(4), 1);
        assert_eq!(Solution::get_reflection(3), 3);
        assert_eq!(Solution::get_reflection(5), 5);
    }

    #[test]
    fn test_leetcode_example_1() {
        let input = [4, 5, 4];
        let expected = [4, 4, 5];
        assert_eq!(
            Solution::sort_by_reflection(input.into()),
            Vec::from(expected)
        )
    }

    #[test]
    fn test_leetcode_example_2() {
        let input = [3, 6, 5, 8];
        let expected = [8, 3, 6, 5];
        assert_eq!(
            Solution::sort_by_reflection(input.into()),
            Vec::from(expected)
        )
    }
}
