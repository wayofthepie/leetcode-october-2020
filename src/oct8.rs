struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(index) => index as i32,
            Err(_) => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    fn example1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let answer = Solution::search(nums, target);
        assert_eq!(answer, -1)
    }
}
