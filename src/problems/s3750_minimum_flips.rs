pub struct Solution;

impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        let n = n as u32;
        let reverse = n.reverse_bits() >> n.leading_zeros();
        (n ^ reverse).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_flips() {
        assert_eq!(Solution::minimum_flips(7), 0);
        assert_eq!(Solution::minimum_flips(10), 4);
    }
}
