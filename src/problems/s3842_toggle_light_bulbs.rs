use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut output = bulbs
            .into_iter()
            .fold(HashMap::new(), |mut acc, bulb| {
                acc.entry(bulb)
                    .and_modify(|val: &mut bool| *val = !*val)
                    .or_insert(true);
                acc
            })
            .into_iter()
            .filter_map(|(k, v)| if v { Some(k) } else { None })
            .collect::<Vec<i32>>();
        output.sort();
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let bulbs = vec![10, 30, 20, 10];
        let expected = vec![20, 30];
        let result = Solution::toggle_light_bulbs(bulbs);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_leetcode_example_2() {
        let bulbs = vec![100, 100];
        let expected = vec![];
        let result = Solution::toggle_light_bulbs(bulbs);
        assert_eq!(result, expected);
    }
}
