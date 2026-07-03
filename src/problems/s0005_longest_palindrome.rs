pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();

        let mut start = 0;
        let mut end = 0;

        let expand_around_center = |mut left: i32, mut right: i32| -> (i32, i32) {
            while left >= 0
                && (right as usize) < chars.len()
                && chars[left as usize] == chars[right as usize]
            {
                left -= 1;
                right += 1;
            }
            (left + 1, right - 1)
        };

        for i in 0..chars.len() {
            let (l1, r1) = expand_around_center(i as i32, i as i32);
            if (r1 - l1) > (end - start) {
                start = l1;
                end = r1;
            }

            let (l2, r2) = expand_around_center(i as i32, i as i32 + 1);
            if (r2 - l2) > (end - start) {
                start = l2;
                end = r2;
            }
        }

        chars[start as usize..=end as usize].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_charater_is_longest_palindrome() {
        let input = String::from("z");
        let output = Solution::longest_palindrome(input);
        assert_eq!(output, "z");
    }

    #[test]
    fn leetcode_example_1() {
        let input = String::from("babad");
        let output = Solution::longest_palindrome(input);
        assert!(output == "bab".to_string() || output == "aba".to_string())
    }

    #[test]
    fn leetcode_example_2() {
        let input = String::from("cbbd");
        let output = Solution::longest_palindrome(input);
        assert_eq!(output, "bb".to_string())
    }
}
