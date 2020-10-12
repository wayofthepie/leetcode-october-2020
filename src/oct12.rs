struct Solution;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() || a.is_empty() {
            return false;
        }
        let normalizer = 'a' as usize;
        let (diff, a_freq, count) = a.chars().zip(b.chars()).fold(
            (vec![0i32; 26], vec![0i32; 26], 0),
            |(mut diff, mut a_freq, mut count), (a, b)| {
                a_freq[a as usize - normalizer] += 1;
                if a != b {
                    diff[a as usize - normalizer] += 1;
                    diff[b as usize - normalizer] -= 1;
                    count += 1;
                }
                (diff, a_freq, count)
            },
        );
        Solution::same_string_and_can_swap(&a, &b, &a_freq)
            || Solution::same_letters_with_two_diffs(&a, &b, &diff, count)
    }

    fn same_string_and_can_swap(a: &str, b: &str, a_freq: &Vec<i32>) -> bool {
        a == b && a_freq.iter().any(|freq| freq >= &2)
    }

    fn same_letters_with_two_diffs(a: &str, b: &str, diff: &Vec<i32>, count: i32) -> bool {
        diff.iter().all(|&freq| freq == 0) && a != b && count == 2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let a = "aaaaaaabc".to_owned();
        let b = "aaaaaaacb".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, true);
    }

    #[test]
    fn example2() {
        let a = "aa".to_owned();
        let b = "aa".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, true);
    }

    #[test]
    fn example3() {
        let a = "abcaa".to_owned();
        let b = "abcbb".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, false);
    }

    #[test]
    fn example4() {
        let a = "ab".to_owned();
        let b = "ab".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, false);
    }

    #[test]
    fn example5() {
        let a = "abcd".to_owned();
        let b = "badc".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, false);
    }

    #[test]
    fn example6() {
        let a = "abab".to_owned();
        let b = "abab".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, true);
    }

    #[test]
    fn example7() {
        let a = "".to_owned();
        let b = "abab".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, false);
    }

    #[test]
    fn example8() {
        let a = "a".to_owned();
        let b = "abab".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, false);
    }

    #[test]
    fn example9() {
        let a = "aaab".to_owned();
        let b = "aaab".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, true);
    }

    #[test]
    fn example10() {
        let a = "aaabbbbd".to_owned();
        let b = "aaabbbbc".to_owned();
        let answer = Solution::buddy_strings(a, b);
        assert_eq!(answer, false);
    }
}
