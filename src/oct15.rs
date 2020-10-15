struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.is_empty() {
            return;
        }
        let length = nums.len() as i32;
        nums.rotate_right((k % length) as usize);
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![1, 2];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn example4() {
        let mut nums = vec![1];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn example5() {
        let mut nums = vec![];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn example6() {
        let mut nums = vec![1, 2, 3];
        Solution::rotate(&mut nums, 4);
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn example7() {
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        Solution::rotate(&mut nums, 11);
        assert_eq!(nums, vec![2, 3, 4, 5, 6, 1]);
    }
}
