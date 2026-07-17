use std::collections::HashSet;

pub struct Solution;

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_left(&mut self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    pub fn turn_right(&mut self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut max_dist = 0;
        let mut cur_position = (0, 0);
        let mut cur_direction = Direction::North;
        let obstacles_set: HashSet<(i32, i32)> =
            obstacles.into_iter().map(|obs| (obs[0], obs[1])).collect();

        'command: for command in commands {
            if command < 0 {
                cur_direction = if command == -2 {
                    cur_direction.turn_left()
                } else if command == -1 {
                    cur_direction.turn_right()
                } else {
                    panic!("Unsupported Direction")
                };
                continue;
            }

            for _ in 0..command {
                let new_pos: (i32, i32) = match cur_direction {
                    Direction::North => (cur_position.0, cur_position.1 + 1),
                    Direction::South => (cur_position.0, cur_position.1 - 1),
                    Direction::East => (cur_position.0 + 1, cur_position.1),
                    Direction::West => (cur_position.0 - 1, cur_position.1),
                };
                if obstacles_set.contains(&new_pos) {
                    continue 'command;
                }

                cur_position = new_pos;
                let cur_dist = cur_position.0.pow(2) + cur_position.1.pow(2);
                max_dist = max_dist.max(cur_dist);
            }
        }
        max_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example_1() {
        let commands = vec![4, -1, 3];
        let obstacles = vec![];
        let expected = 25;
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_leetcode_example_2() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        let expected = 65;
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_leetcode_example_3() {
        let commands = vec![-1, 9];
        let obstacles = vec![vec![5, 0], vec![2, 0]];
        let expected = 1;
        let result = Solution::robot_sim(commands, obstacles);
        assert_eq!(result, expected);
    }
}
