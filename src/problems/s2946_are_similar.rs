use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let (n, m) = (mat.len(), mat[0].len());
        let k = (k as usize) % m;
        let mut modified = mat.clone();

        for _ in 0..k {
            for row in 0..n {
                if row % 2 == 0 {
                    let tmp = modified[row].clone();
                    let mut tmp = VecDeque::from(tmp);
                    let el = tmp.pop_front().unwrap();
                    tmp.push_back(el);
                    modified[row] = Vec::from(tmp);
                } else {
                    let tmp = modified[row].clone();
                    let mut tmp = VecDeque::from(tmp);
                    let el = tmp.pop_back().unwrap();
                    tmp.push_front(el);
                    modified[row] = Vec::from(tmp);
                }
            }
        }

        mat == modified
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 4;
        assert!(!Solution::are_similar(mat.into(), k))
    }

    #[test]
    fn test_leetcode_example_2() {
        let mat = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        let k = 2;
        assert!(Solution::are_similar(mat.into(), k))
    }

    #[test]
    fn test_leetcode_example_3() {
        let mat = vec![vec![2, 2], vec![2, 2]];
        let k = 3;
        assert!(Solution::are_similar(mat.into(), k))
    }
}
