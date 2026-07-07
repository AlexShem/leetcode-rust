pub struct Solution;

impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut count = 0;
        while num1 > 0 && num2 > 0 {
            if num1 >= num2 {
                num1 -= num2;
            } else {
                num2 -= num1;
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_operations() {
        assert_eq!(Solution::count_operations(2, 3), 3);
        assert_eq!(Solution::count_operations(10, 10), 1);
    }
}
