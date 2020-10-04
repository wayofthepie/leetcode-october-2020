use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut skip = HashSet::new();
        for i in 0..intervals.len() {
            for j in 0..intervals.len() {
                if skip.contains(&j) || i == j {
                    continue;
                }
                if intervals[i][0] <= intervals[j][0] && intervals[i][1] >= intervals[j][1] {
                    skip.insert(j);
                }
            }
        }
        (intervals.len() - skip.len()) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let intervals = vec![vec![1, 4], vec![3, 6], vec![2, 8]];
        let answer = Solution::remove_covered_intervals(intervals);
        assert_eq!(answer, 2);
    }

    #[test]
    fn example2() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        let answer = Solution::remove_covered_intervals(intervals);
        assert_eq!(answer, 1);
    }

    #[test]
    fn example3() {
        let intervals = vec![vec![0, 10], vec![5, 12]];
        let answer = Solution::remove_covered_intervals(intervals);
        assert_eq!(answer, 2);
    }

    #[test]
    fn example4() {
        let intervals = vec![vec![3, 10], vec![4, 10], vec![5, 11]];
        let answer = Solution::remove_covered_intervals(intervals);
        assert_eq!(answer, 2);
    }

    #[test]
    fn example5() {
        let intervals = vec![vec![1, 2], vec![1, 4], vec![3, 4]];
        let answer = Solution::remove_covered_intervals(intervals);
        assert_eq!(answer, 1);
    }
}
