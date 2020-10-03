use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        nums.sort_unstable();
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[j] - nums[i] == k {
                    set.insert((nums[i], nums[j]));
                }
            }
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let nums = vec![3, 1, 4, 1, 5];
        let k = 2;
        assert_eq!(Solution::find_pairs(nums, k), 2);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 1;
        assert_eq!(Solution::find_pairs(nums, k), 4);
    }

    #[test]
    fn example3() {
        let nums = vec![1, 3, 1, 5, 4];
        let k = 0;
        assert_eq!(Solution::find_pairs(nums, k), 1);
    }

    #[test]
    fn example4() {
        let nums = vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3];
        let k = 3;
        assert_eq!(Solution::find_pairs(nums, k), 2);
    }

    #[test]
    fn example5() {
        let nums = vec![-1, -2, -3];
        let k = 1;
        assert_eq!(Solution::find_pairs(nums, k), 2);
    }
}
