struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.iter().any(|row| row.contains(&target))
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
        ];
        assert!(Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn example2() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 14, 15, 16],
        ];
        assert!(!Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn example3() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 14, 15, 16],
        ];
        assert!(Solution::search_matrix(matrix, 1));
    }

    #[test]
    fn example4() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 14, 15, 16],
        ];
        assert!(Solution::search_matrix(matrix, 14));
    }

    #[test]
    fn example5() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 14, 15, 16],
        ];
        assert!(Solution::search_matrix(matrix, 8));
    }

    #[test]
    fn example6() {
        let matrix = vec![vec![1], vec![6], vec![11]];
        assert!(!Solution::search_matrix(matrix, 0));
    }

    #[test]
    fn example7() {
        let matrix = vec![vec![], vec![], vec![]];
        assert!(!Solution::search_matrix(matrix, 0));
    }

    #[test]
    fn example8() {
        let matrix = vec![vec![1], vec![3]];
        assert!(!Solution::search_matrix(matrix, 4));
    }

    #[test]
    fn example9() {
        let matrix = vec![vec![1], vec![3]];
        assert!(!Solution::search_matrix(matrix, 0));
    }

    #[test]
    fn example10() {
        let matrix = vec![vec![1], vec![3]];
        assert!(!Solution::search_matrix(matrix, 2));
    }

    #[test]
    fn example11() {
        let matrix = vec![vec![1], vec![3], vec![5]];
        assert!(!Solution::search_matrix(matrix, 4));
    }
}
