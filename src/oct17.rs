struct Solution;

use std::collections::HashMap;

const SEQUENCE_LENGTH: usize = 10;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let chars = s.chars().collect::<Vec<char>>();
        let length = chars.len();
        if length < 10 {
            return Vec::new();
        }
        let mut sequences = HashMap::new();
        let mut start = 0;
        while chars.len() - start > SEQUENCE_LENGTH - 1 {
            *sequences.entry(&chars[start..=start + 9]).or_insert(0) += 1;
            start += 1;
        }
        sequences
            .iter()
            .filter_map(|(&k, &v)| {
                if v > 1 {
                    Some(k.iter().collect::<String>())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let dna = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected = vec!["AAAAACCCCC".to_owned(), "CCCCCAAAAA".to_owned()];
        assert_eq!(answer, expected);
    }

    #[test]
    fn example2() {
        let dna = "AAAAACCCCCAAAAAGGGTTTAAAAACCCCCC".to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected = vec!["AAAAACCCCC".to_owned()];
        assert_eq!(answer, expected);
    }

    #[test]
    fn example3() {
        let dna = "GGGTTAAAAACCCCCTTTTTTTAAAAACCCTCCCAAAAAGGGTTT".to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected = vec!["TTAAAAACCC"];
        assert_eq!(answer, expected);
    }

    #[test]
    fn example4() {
        let dna = "".to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected: Vec<String> = Vec::new();
        assert_eq!(answer, expected);
    }

    #[test]
    fn example5() {
        let dna = "AAAAACCCCCAAAAAGGGTTTAAAAACCCCCCAAAAAAAAAATCTCTCTCGGGGGGGGAAAAAAAAAA".to_owned();

        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected = vec![
            "AAAAAAAAAA".to_owned(),
            "AAAAACCCCC".to_owned(),
            "CCCCCAAAAA".to_owned(),
        ];
        assert_eq!(answer, expected);
    }

    #[test]
    fn example6() {
        let dna = "AAAAAAAAAA".to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected: Vec<String> = Vec::new();
        assert_eq!(answer, expected);
    }

    #[test]
    fn example7() {
        let dna = "AAAAAAAAAAAAAAAA".to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected = vec!["AAAAAAAAAA".to_owned()];
        assert_eq!(answer, expected);
    }

    #[test]
    fn example8() {
        let dna = include_str!("../resources/oct17_dna_large.txt").to_owned();
        let mut answer = Solution::find_repeated_dna_sequences(dna);
        answer.sort();
        let expected: Vec<String> = vec![];
        assert_eq!(answer, expected);
    }
}
