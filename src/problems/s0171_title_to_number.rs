pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .fold(0, |acc, ch| acc * 26 + (ch as i32 - 'A' as i32 + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(Solution::title_to_number("A".into()), 1);
        assert_eq!(Solution::title_to_number("AB".into()), 28);
        assert_eq!(Solution::title_to_number("ZY".into()), 701);
    }
}
