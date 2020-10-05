struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut mask = 1;
        let steps: i32 = (n as f64).log2() as i32 + 1;
        for i in 1..=steps {
            mask = 2i32.pow(i as u32) - 1;
        }
        n ^ mask
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let n = 5;
        let answer = Solution::bitwise_complement(n);
        assert_eq!(answer, 2);
    }

    #[test]
    fn example2() {
        let n = 7;
        let answer = Solution::bitwise_complement(n);
        assert_eq!(answer, 0);
    }

    #[test]
    fn example3() {
        let n = 10;
        let answer = Solution::bitwise_complement(n);
        assert_eq!(answer, 5);
    }

    #[test]
    fn example4() {
        let n = 1453689;
        let answer = Solution::bitwise_complement(n);
        assert_eq!(answer, 643462);
    }
}
