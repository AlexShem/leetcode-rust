use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }
        let k = (k as usize).min(nums.len() - 1);

        let (mut l, mut r) = (0usize, k);
        let mut set = HashSet::with_capacity(k + 1);
        for num in &nums[l..=r] {
            if !set.insert(num) {
                return true;
            }
        }

        l += 1;
        r += 1;
        while r < nums.len() {
            set.remove(&nums[l - 1]);
            if !set.insert(&nums[r]) {
                return true;
            }
            l += 1;
            r += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}
