use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![i32::MAX; n + 2];
        dp[n] = 0;
        dp[n + 1] = 0;

        for i in (0..n).rev() {
            dp[i] = cost[i] + min(dp[i + 1], dp[i + 2]);
        }
        min(dp[0], dp[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let cost = [10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost.into()), 15);
    }

    #[test]
    fn test_leetcode_example_2() {
        let cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost.into()), 6);
    }
}
