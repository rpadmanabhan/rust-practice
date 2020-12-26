use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct AlignResult {
    pub edit_dist: u32,
    pub cigar: String,
}

// TO DO: Return cigar string from alignment
impl AlignResult {
    pub fn new(
        edit_dist: u32,
        cigar_ops: Vec<u8>,
        cigar_ops_counts: Vec<u32>,
    ) -> Result<AlignResult, &'static str> {
        let cigar_ops_to_char: HashMap<u8, char> =
            [(0, 'M'), (1, 'I'), (2, 'D')].iter().cloned().collect();

        let mut cigar = String::with_capacity(1 * cigar_ops.len() + 4 * cigar_ops_counts.len());
        for (i, c) in cigar_ops.iter().enumerate() {
            cigar.push_str(&cigar_ops_counts[i].to_string());
            cigar.push(cigar_ops_to_char[c]);
        }

        Ok(AlignResult { edit_dist, cigar })
    }
}

pub fn edit_dist(s1: &str, s2: &str) -> Option<u32> {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    // initialize DP matrix
    let mut dp_mat = vec![vec![0u32; s2.len() + 1]; s1.len() + 1];
    dp_mat[0] = (0u32..u32::try_from(s2.len()).unwrap() + 1).collect::<Vec<u32>>();
    let mut i = 0;
    for row in &mut dp_mat {
        for col in &mut *row {
            *col = i;
            i += 1;
            break;
        }
    }

    // Track alignment cigar
    let mut i: usize = 1;
    let mut cigar_idx: usize = 0;
    let mut cigar_ops: Vec<u8> = Vec::new();
    let mut cigar_ops_counts: Vec<u32> = Vec::new();

    // traverse DP matrix - row wise
    while i < s1.len() + 1 {
        let mut j: usize = 1;

        while j < s2.len() + 1 {
            // Match / Mismatch
            let mut delta = 0;
            if s1_bytes[i - 1] != s2_bytes[j - 1] {
                delta = 1;
            }

            // Calculate lowest cost move
            let mut move_choices = vec![
                (0, dp_mat[i - 1][j - 1] + delta),
                (1, dp_mat[i - 1][j] + 1),
                (2, dp_mat[i][j - 1] + 1),
            ];
            move_choices.sort_by_key(|k| k.1);

            // Update DP Matrix
            dp_mat[i][j] = move_choices[0].1;

            // Update Cigar
            let cigar_op = move_choices[0].0;
            if (cigar_idx > 0) && (cigar_op == cigar_ops[cigar_idx - 1]) {
                cigar_ops_counts[cigar_idx - 1] += 1;
            } else {
                cigar_ops.push(cigar_op);
                cigar_ops_counts.push(1);
                cigar_idx += 1;
            }
            j += 1;
        }
        i += 1;
    }

    // Return Editdistance
    Some(dp_mat[s1.len()][s2.len()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_length() {
        let s1 = "ACGTA";
        let s2 = "GCGTA";
        assert_eq!(std::option::Option::Some(1), edit_dist(s1, s2))
    }

    #[test]
    fn diff_length() {
        let s1 = "ACGTAAACAC";
        let s2 = "ACGTAACAC";
        assert_eq!(std::option::Option::Some(1), edit_dist(s1, s2))
    }

    #[test]
    fn large_del() {
        let s1 = "ACGTAAAAACCCAGGGCACACGTGGGGCACACACA";
        let s2 = "ACGTCACACGTGGGGCACACACA";
        assert_eq!(std::option::Option::Some(12), edit_dist(s1, s2))
    }

    #[test]
    fn large_ins() {
        let s1 = "ACGTCACACGTGGGGCACACACAGGGGTTGTGTG";
        let s2 = "ACGTCACACGTGGGGCACACACA";
        assert_eq!(std::option::Option::Some(11), edit_dist(s1, s2))
    }

    #[test]
    fn ins_del_mismatch() {
        let s1 = "ACGTCACACGTGGGGCACACACAGGGGTTGTGTG";
        let s2 = "ATGTCACACGTGGGGCACACA";
        assert_eq!(std::option::Option::Some(14), edit_dist(s1, s2))
    }
}
