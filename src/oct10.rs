struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
        let mut current = &points[0];
        points[1..].iter().fold(1, |n, point| {
            if point[0] > current[1] {
                current = point;
                n + 1
            } else {
                n
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn paper_example() {
        let points = vec![vec![1, 3], vec![2, 3], vec![3, 4], vec![4, 5]];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 2)
    }

    #[test]
    fn example1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 2)
    }

    #[test]
    fn example2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 4)
    }

    #[test]
    fn example3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 2)
    }

    #[test]
    fn example4() {
        let points = vec![vec![1, 2]];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 1)
    }

    #[test]
    fn example5() {
        let points = vec![vec![2, 3], vec![3, 2]];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 1)
    }

    #[test]
    fn example6() {
        let points = vec![
            vec![3, 9],
            vec![7, 12],
            vec![3, 8],
            vec![6, 8],
            vec![9, 10],
            vec![2, 9],
            vec![0, 9],
            vec![3, 9],
            vec![0, 6],
            vec![2, 8],
        ];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 2)
    }

    #[test]
    fn example7() {
        let points = vec![
            vec![9, 12],
            vec![1, 10],
            vec![4, 11],
            vec![8, 12],
            vec![3, 9],
            vec![6, 9],
            vec![6, 7],
        ];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 2);
    }

    #[test]
    fn should_handle_empty_case() {
        let points = vec![];
        let answer = Solution::find_min_arrow_shots(points);
        assert_eq!(answer, 0);
    }
}
