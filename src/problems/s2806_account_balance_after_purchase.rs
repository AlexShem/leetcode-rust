pub struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let rounded_up = ((purchase_amount + 5) / 10) * 10;
        100 - rounded_up
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_balance_after_purchase() {
        assert_eq!(Solution::account_balance_after_purchase(9), 90);
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
        assert_eq!(Solution::account_balance_after_purchase(10), 90);
        assert_eq!(Solution::account_balance_after_purchase(0), 100);
        assert_eq!(Solution::account_balance_after_purchase(100), 0);
    }
}
