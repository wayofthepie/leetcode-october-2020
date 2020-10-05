struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Solution::combination_sum_rec(&candidates, vec![], target)
    }

    fn combination_sum_rec(candidates: &[i32], captured: Vec<i32>, sum: i32) -> Vec<Vec<i32>> {
        if sum == 0 {
            return vec![captured];
        }
        candidates.iter().fold(Vec::new(), |mut acc, n| {
            if n <= &sum && n >= captured.last().unwrap_or(&0) {
                let mut c = captured.clone();
                c.push(*n);
                acc.append(&mut Solution::combination_sum_rec(candidates, c, sum - n));
            }
            acc
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let candidates = vec![7, 2, 3];
        let target = 7;
        let mut answer = Solution::combination_sum(candidates, target);
        answer.sort();
        assert_eq!(answer, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn example2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let mut answer = Solution::combination_sum(candidates, target);
        answer.sort();
        assert_eq!(answer, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    #[test]
    fn example3() {
        let candidates = vec![2];
        let target = 1;
        let mut answer = Solution::combination_sum(candidates, target);
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example4() {
        let candidates = vec![1];
        let target = 2;
        let mut answer = Solution::combination_sum(candidates, target);
        answer.sort();
        assert_eq!(answer, vec![vec![1, 1]]);
    }

    #[test]
    fn example5() {
        let candidates = vec![1];
        let target = 1;
        let mut answer = Solution::combination_sum(candidates, target);
        answer.sort();
        assert_eq!(answer, vec![vec![1]]);
    }
}
